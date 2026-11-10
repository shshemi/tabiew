use crate::{
    AppResult,
    args::Args,
    duckdb::{helper::quote, ingester::FileIngester, session::SessionConn},
    misc::available_col_name::available_col_name,
};
use duckdb::params;
use serde::Serialize;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
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
}

impl FwfIngester {
    pub fn new(path: PathBuf, options: FwfOptions) -> Self {
        Self { path, options }
    }
}

impl FileIngester for FwfIngester {
    fn ingest(&mut self, conn: SessionConn, alias: &str) -> AppResult<()> {
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
        let columns = match &self.options.widths {
            Some(widths) => fixed(widths, self.options.separator_length),
            None => {
                let sample = conn
                    .prepare(&format!("SELECT line FROM ({LINES}) LIMIT {SAMPLE}"))?
                    .query_map(params![path, DELIM, skip], |row| row.get::<_, String>(0))?
                    .collect::<Result<Vec<_>, _>>()?;
                let mut blank = Vec::<bool>::new();
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
                let candidates = (0..blank.len()).filter(|&i| blank[i]).collect::<Vec<_>>();
                if !candidates.is_empty() {
                    let checks = candidates
                        .iter()
                        .map(|i| {
                            format!(
                                "coalesce(bool_and(substr(line, {}, 1) IN (' ', '')), true)",
                                i + 1
                            )
                        })
                        .collect::<Vec<_>>();
                    let verified = conn.query_row(
                        &format!("SELECT {} FROM ({LINES})", checks.join(", ")),
                        params![path, DELIM, skip],
                        |row| {
                            (0..candidates.len())
                                .map(|j| row.get::<_, bool>(j))
                                .collect::<Result<Vec<_>, _>>()
                        },
                    )?;
                    for (&i, gap) in candidates.iter().zip(verified) {
                        blank[i] = gap;
                    }
                }
                let columns = inferred(&blank);
                match &header {
                    Some(header) => merge_unnamed(columns, header),
                    None => columns,
                }
            }
        };
        if columns.is_empty() {
            return Ok(());
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
            .zip(&names)
            .map(|(&(start, len), name)| {
                let len = len.map(|len| format!(", {len}")).unwrap_or_default();
                format!(
                    "NULLIF(trim(substr(line, {start}{len})), '') AS {}",
                    quote(name)
                )
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
