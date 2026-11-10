use crate::{
    AppResult,
    args::Args,
    duckdb::{
        ingester::{FileIngester, IngestProgress, progress::StepProgress},
        session::SessionConn,
        sql::quote,
    },
    misc::available_col_name::available_col_name,
};
use duckdb::{Connection, params};
use serde::Serialize;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::Arc,
};

const SAMPLE: usize = 20_480;
const DELIM: &str = "\u{1f}";
const LINES: &str = "SELECT line FROM read_csv(?, columns = {'line': 'VARCHAR'}, header = false, delim = ?, quote = '', escape = '', auto_detect = false, skip = ?) WHERE trim(line) <> ''";

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FwfOptions {
    pub widths: Option<Vec<usize>>,
    pub separator_length: usize,
    pub flexible_width: bool,
    pub header: bool,
}

impl Default for FwfOptions {
    fn default() -> Self {
        Self {
            widths: None,
            separator_length: 1,
            flexible_width: true,
            header: true,
        }
    }
}

impl FwfOptions {
    pub fn from_args(args: &Args) -> Self {
        Self {
            widths: args
                .widths
                .split(',')
                .map(|width| width.trim().parse())
                .collect::<Result<_, _>>()
                .ok(),
            separator_length: args.separator_length,
            flexible_width: !args.no_flexible_width,
            header: !args.no_header,
        }
    }
}

pub struct FwfIngester {
    path: PathBuf,
    options: FwfOptions,
    types: Vec<String>,
    progress: Arc<StepProgress>,
}

impl FwfIngester {
    pub fn new(path: PathBuf, options: FwfOptions, types: Vec<String>) -> Self {
        let steps = if options.widths.is_some() && types.is_empty() {
            1
        } else {
            2
        };
        Self {
            path,
            options,
            types,
            progress: StepProgress::new(steps),
        }
    }
}

impl FileIngester for FwfIngester {
    fn progress(&self) -> Arc<dyn IngestProgress> {
        self.progress.clone()
    }
    fn ingest(&mut self, conn: SessionConn, alias: &str) -> AppResult<()> {
        self.progress.start(&conn);
        let path = self.path.to_string_lossy().into_owned();
        let skip = self.options.header as i64;
        let header = if self.options.header {
            BufReader::new(File::open(&self.path)?)
                .lines()
                .next()
                .transpose()?
                .map(|line| line.trim_end_matches('\r').to_owned())
        } else {
            None
        };
        let sample = if self.options.widths.is_none() || !self.types.is_empty() {
            conn.prepare(&format!("SELECT line FROM ({LINES}) LIMIT {SAMPLE}"))?
                .query_map(params![path, DELIM, skip], |row| row.get::<_, String>(0))?
                .collect::<Result<Vec<_>, _>>()?
        } else {
            Vec::new()
        };
        let mut blank = Vec::<bool>::new();
        if self.options.widths.is_none() {
            for line in header.iter().chain(&sample) {
                for (i, c) in line.chars().enumerate() {
                    if i == blank.len() {
                        blank.push(true);
                    }
                    if c != ' ' {
                        blank[i] = false;
                    }
                }
            }
        }
        let columns = match &self.options.widths {
            Some(widths) => fixed(widths, self.options.separator_length),
            None => shape(&blank, header.as_deref()),
        };
        if columns.is_empty() {
            return Ok(());
        }
        let lines = Lines {
            conn: &conn,
            path: &path,
            skip,
        };
        let types = &self.types;
        let gaps = (0..blank.len()).filter(|&i| blank[i]).collect::<Vec<_>>();
        let typed = lines.sampled(&columns, types)?;
        let (gap_checks, kinds) = lines.verify(&gaps, &columns, &typed)?;
        let (columns, kinds) = if gap_checks.contains(&false) {
            for (&i, &gap) in gaps.iter().zip(&gap_checks) {
                blank[i] = gap;
            }
            let reshaped = shape(&blank, header.as_deref());
            let changed = reshaped
                .iter()
                .filter(|column| !columns.contains(column))
                .copied()
                .collect::<Vec<_>>();
            let typed = lines.sampled(&changed, types)?;
            let (_, changed_kinds) = lines.verify(&[], &changed, &typed)?;
            let kinds = reshaped
                .iter()
                .map(|column| match columns.iter().position(|c| c == column) {
                    Some(i) => kinds[i].clone(),
                    None => {
                        changed_kinds[changed.iter().position(|c| c == column).unwrap()].clone()
                    }
                })
                .collect();
            (reshaped, kinds)
        } else {
            (columns, kinds)
        };
        if self.options.widths.is_none() || !types.is_empty() {
            self.progress.next();
        }
        let mut names = Vec::<String>::new();
        for (i, &(start, len)) in columns.iter().enumerate() {
            let name = match &header {
                Some(header) => header
                    .chars()
                    .skip(start - 1)
                    .take(len.unwrap_or(usize::MAX))
                    .collect::<String>()
                    .trim()
                    .to_owned(),
                None => format!("column_{}", i + 1),
            };
            let name = available_col_name(&name, |name| !names.iter().any(|n| n == name));
            names.push(name);
        }
        let select = columns
            .iter()
            .zip(&kinds)
            .zip(&names)
            .map(|((&(start, len), kind), name)| match kind {
                Some(kind) => format!("CAST({} AS {kind}) AS {}", cut(start, len), quote(name)),
                None => format!("{} AS {}", cut(start, len), quote(name)),
            })
            .collect::<Vec<_>>();
        let strict = if self.options.flexible_width {
            String::new()
        } else {
            let (start, len) = columns[columns.len() - 1];
            let min = start - 1 + len.unwrap_or(1);
            format!(
                " WHERE CASE WHEN length(line) < {min} THEN error('Line is shorter than {min} characters: ' || line) ELSE true END"
            )
        };
        conn.execute(
            &format!(
                "CREATE OR REPLACE TABLE {}.data AS SELECT {} FROM ({LINES}){strict}",
                quote(alias),
                select.join(", ")
            ),
            params![path, DELIM, skip],
        )?;
        Ok(())
    }
}

type Column = (usize, Option<usize>);

struct Lines<'a> {
    conn: &'a Connection,
    path: &'a str,
    skip: i64,
}

impl Lines<'_> {
    fn bools(&self, sql: &str, count: usize) -> AppResult<Vec<bool>> {
        if count == 0 {
            return Ok(Vec::new());
        }
        Ok(self
            .conn
            .query_row(sql, params![self.path, DELIM, self.skip], |row| {
                (0..count)
                    .map(|i| row.get::<_, bool>(i))
                    .collect::<Result<Vec<_>, _>>()
            })?)
    }
    fn sampled(&self, columns: &[Column], types: &[String]) -> AppResult<Vec<(Column, String)>> {
        let pairs = columns
            .iter()
            .flat_map(|&column| types.iter().map(move |kind| (column, kind.clone())))
            .collect::<Vec<_>>();
        let checks = pairs
            .iter()
            .map(|((start, len), kind)| type_check(kind, &cut(*start, *len)))
            .collect::<Vec<_>>();
        let passed = self.bools(
            &format!(
                "SELECT {} FROM (SELECT line FROM ({LINES}) LIMIT {SAMPLE})",
                checks.join(", ")
            ),
            checks.len(),
        )?;
        Ok(pairs
            .into_iter()
            .zip(passed)
            .filter_map(|(pair, passed)| passed.then_some(pair))
            .collect())
    }
    fn verify(
        &self,
        gaps: &[usize],
        columns: &[Column],
        typed: &[(Column, String)],
    ) -> AppResult<(Vec<bool>, Vec<Option<String>>)> {
        let checks = gaps
            .iter()
            .map(|i| {
                format!(
                    "coalesce(bool_and(substr(line, {}, 1) IN (' ', '')), true)",
                    i + 1
                )
            })
            .chain(
                typed
                    .iter()
                    .map(|((start, len), kind)| type_check(kind, &cut(*start, *len))),
            )
            .collect::<Vec<_>>();
        let mut passed = self.bools(
            &format!("SELECT {} FROM ({LINES})", checks.join(", ")),
            checks.len(),
        )?;
        let type_checks = passed.split_off(gaps.len());
        let kinds = columns
            .iter()
            .map(|column| {
                typed
                    .iter()
                    .zip(&type_checks)
                    .find(|((typed, _), passed)| typed == column && **passed)
                    .map(|((_, kind), _)| kind.clone())
            })
            .collect();
        Ok((passed, kinds))
    }
}

fn cut(start: usize, len: Option<usize>) -> String {
    let len = len.map(|len| format!(", {len}")).unwrap_or_default();
    format!("NULLIF(trim(substr(line, {start}{len})), '')")
}

fn type_check(kind: &str, value: &str) -> String {
    let check = match kind {
        "BIGINT" => format!(
            "regexp_full_match({value}, '[+-]?[0-9]+') AND TRY_CAST({value} AS BIGINT) IS NOT NULL"
        ),
        "BOOLEAN" => format!("lower({value}) IN ('true', 'false')"),
        "DATE" => format!(
            "regexp_full_match({value}, '[0-9]{{4}}-[0-9]{{2}}-[0-9]{{2}}') AND TRY_CAST({value} AS DATE) IS NOT NULL"
        ),
        _ => format!("TRY_CAST({value} AS {kind}) IS NOT NULL"),
    };
    format!("coalesce(bool_and({value} IS NULL OR ({check})) AND count({value}) > 0, false)")
}

fn shape(blank: &[bool], header: Option<&str>) -> Vec<(usize, Option<usize>)> {
    let columns = inferred(blank);
    match header {
        Some(header) => merge_unnamed(columns, header),
        None => columns,
    }
}

fn fixed(widths: &[usize], separator_length: usize) -> Vec<(usize, Option<usize>)> {
    widths
        .iter()
        .scan(1, |start, &width| {
            let column = (*start, Some(width));
            *start += width + separator_length;
            Some(column)
        })
        .collect()
}

fn inferred(blank: &[bool]) -> Vec<(usize, Option<usize>)> {
    let mut columns = Vec::new();
    let mut start = None;
    for (i, &blank) in blank.iter().chain([&true]).enumerate() {
        match (start, blank) {
            (None, false) => start = Some(i),
            (Some(s), true) => {
                columns.push((s + 1, Some(i - s)));
                start = None;
            }
            _ => {}
        }
    }
    if let Some(last) = columns.last_mut() {
        last.1 = None;
    }
    columns
}

fn merge_unnamed(
    columns: Vec<(usize, Option<usize>)>,
    header: &str,
) -> Vec<(usize, Option<usize>)> {
    let header = header.chars().collect::<Vec<_>>();
    let named = |start: usize, len: Option<usize>| {
        header
            .iter()
            .skip(start - 1)
            .take(len.unwrap_or(usize::MAX))
            .any(|c| *c != ' ')
    };
    let mut merged = Vec::<(usize, Option<usize>)>::new();
    let mut lead = None;
    for (start, len) in columns.iter().copied() {
        match merged.last_mut() {
            Some(last) if !named(start, len) => last.1 = len.map(|len| start + len - last.0),
            _ if !named(start, len) => {
                lead.get_or_insert(start);
            }
            _ => {
                let first = lead.take().unwrap_or(start);
                merged.push((first, len.map(|len| start + len - first)));
            }
        }
    }
    if merged.is_empty() { columns } else { merged }
}
