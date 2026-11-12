use crate::{
    AppResult,
    collections::ragged_vec::RaggedVec,
    duckdb::{
        DuckDB,
        cancel::{self, Cancel},
        ingester::progress::StepProgress,
        sql::quote,
    },
    misc::{config::config, workers},
};
use anyhow::anyhow;
use duckdb::{
    Connection,
    arrow::array::{Array, Float64Array, Int64Array, StringArray},
};
use ratatui::widgets::Bar;
use std::{collections::BTreeMap, sync::Arc};
use tokio::sync::oneshot::{self, Receiver, error::TryRecvError};

#[derive(Debug, Clone)]
pub enum PlotRequest {
    Histogram {
        column: String,
        buckets: usize,
        numeric: bool,
    },
    Scatter {
        x: String,
        y: String,
        group: Option<String>,
    },
}

pub enum PlotData {
    Histogram(Vec<(String, u64)>),
    Scatter {
        points: RaggedVec<(f64, f64)>,
        x: String,
        y: String,
        groups: Option<Vec<String>>,
    },
}

pub(super) struct Plotting {
    pub(super) title: String,
    pub(super) progress: Arc<StepProgress>,
    view: String,
    result: Receiver<AppResult<PlotData>>,
    cancel: Cancel,
}

impl DuckDB {
    pub fn plot(&mut self, view: &str, request: PlotRequest) -> AppResult<()> {
        let conn = self.session.session()?;
        conn.set_query_progress_tracking(true)?;
        let progress = StepProgress::new(1);
        progress.start(&conn);
        let title = match &request {
            PlotRequest::Histogram { column, .. } => format!("Plotting histogram of {column}"),
            PlotRequest::Scatter { x, y, .. } => format!("Plotting {x} against {y}"),
        };
        let cancel = Cancel::default();
        cancel.watch(&conn);
        let job = cancel.clone();
        let (sender, result) = oneshot::channel();
        let target = view.to_string();
        workers::spawn_short(move || {
            let _ = sender.send(job.scoped(|| match request {
                PlotRequest::Histogram {
                    column,
                    buckets,
                    numeric,
                } => histogram(&conn, &target, &column, buckets, numeric),
                PlotRequest::Scatter { x, y, group } => scatter(&conn, &target, x, y, group),
            }));
        });
        self.plotting.push(Plotting {
            title,
            progress,
            view: view.to_string(),
            result,
            cancel,
        });
        Ok(())
    }
    pub(super) fn cancel_plots(&mut self, view: &str) {
        for plotting in self
            .plotting
            .iter()
            .filter(|plotting| plotting.view == view)
        {
            plotting.cancel.cancel();
        }
    }
    pub(super) fn poll_plots(&mut self) -> Vec<(String, AppResult<PlotData>)> {
        let mut done = Vec::new();
        let mut i = 0;
        while i < self.plotting.len() {
            let result = match self.plotting[i].result.try_recv() {
                Err(TryRecvError::Empty) => {
                    i += 1;
                    continue;
                }
                Ok(result) => result,
                Err(TryRecvError::Closed) => Err(anyhow!("Plot stopped without a result")),
            };
            let plotting = self.plotting.swap_remove(i);
            if !plotting.cancel.cancelled() {
                done.push((plotting.view, result));
            }
        }
        done
    }
}

const VALUE_BYTES: usize = size_of::<(String, u64)>() + size_of::<Bar<'static>>();

fn budget() -> usize {
    config().process_ram_size().as_u64() as usize
}

fn too_big(what: &str) -> anyhow::Error {
    anyhow!(
        "{what} in process_ram_size ({}). Raise process_ram_size in the config",
        config().process_ram_size()
    )
}

fn unquote(quoted: &str) -> String {
    quoted.trim_matches('"').replace("\"\"", "\"")
}

fn histogram(
    conn: &Connection,
    view: &str,
    column: &str,
    buckets: usize,
    numeric: bool,
) -> AppResult<PlotData> {
    let (view, column) = (quote(view), quote(column));
    if !numeric {
        let mut statement = conn.prepare(&format!(
            "SELECT coalesce(CAST({column} AS VARCHAR), 'null'), count(*) AS n FROM {view} GROUP BY 1 ORDER BY n DESC"
        ))?;
        let mut data = Vec::new();
        let mut bytes = 0;
        for batch in statement.stream_arrow([])? {
            cancel::check()?;
            let labels = batch
                .column(0)
                .as_any()
                .downcast_ref::<StringArray>()
                .ok_or_else(|| anyhow!("Unexpected label type"))?;
            let counts = batch
                .column(1)
                .as_any()
                .downcast_ref::<Int64Array>()
                .ok_or_else(|| anyhow!("Unexpected count type"))?;
            for row in 0..batch.num_rows() {
                let label = labels.value(row);
                bytes += label.len() + VALUE_BYTES;
                if bytes > budget() {
                    return Err(too_big(&format!(
                        "{} has more distinct values than fit",
                        unquote(&column)
                    )));
                }
                data.push((label.to_string(), counts.value(row) as u64));
            }
        }
        return Ok(PlotData::Histogram(data));
    }
    if buckets == 0 {
        return Err(anyhow!("Bucket count must be greater than zero"));
    }
    let (min, max): (Option<f64>, Option<f64>) = conn.query_row(
        &format!("SELECT min({column})::DOUBLE, max({column})::DOUBLE FROM {view}"),
        [],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )?;
    let (Some(min), Some(max)) = (min, max) else {
        return Err(anyhow!("No value found"));
    };
    let width = (max - min) / buckets as f64;
    let index = match width > 0.0 {
        true => format!(
            "least(floor(({column}::DOUBLE - {min}) / {width})::BIGINT, {})",
            buckets - 1
        ),
        false => "0".to_string(),
    };
    let mut counts = vec![0; buckets];
    for row in conn
        .prepare(&format!(
            "SELECT {index}, count(*) FROM {view} WHERE {column} IS NOT NULL GROUP BY 1"
        ))?
        .query_map([], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, i64>(1)?)))?
    {
        let (bucket, count) = row?;
        counts[(bucket.max(0) as usize).min(buckets - 1)] += count as u64;
    }
    Ok(PlotData::Histogram(
        crate::tui::plots::histogram_plot::bucket_labels(min, max, counts),
    ))
}

fn scatter(
    conn: &Connection,
    view: &str,
    x: String,
    y: String,
    group: Option<String>,
) -> AppResult<PlotData> {
    let group_select = match &group {
        Some(group) => format!(", coalesce(CAST({} AS VARCHAR), 'null')", quote(group)),
        None => String::new(),
    };
    let mut statement = conn.prepare(&format!(
        "SELECT {}::DOUBLE, {}::DOUBLE{group_select} FROM {} WHERE {} IS NOT NULL AND {} IS NOT NULL",
        quote(&x),
        quote(&y),
        quote(view),
        quote(&x),
        quote(&y),
    ))?;
    let mut groups = BTreeMap::<String, Vec<(f64, f64)>>::new();
    let mut all = Vec::new();
    let mut points_read = 0;
    for batch in statement.stream_arrow([])? {
        cancel::check()?;
        let xs = batch
            .column(0)
            .as_any()
            .downcast_ref::<Float64Array>()
            .ok_or_else(|| anyhow!("{x} is not numeric"))?;
        let ys = batch
            .column(1)
            .as_any()
            .downcast_ref::<Float64Array>()
            .ok_or_else(|| anyhow!("{y} is not numeric"))?;
        let names = batch
            .columns()
            .get(2)
            .and_then(|column| column.as_any().downcast_ref::<StringArray>());
        points_read += batch.num_rows();
        if points_read * size_of::<(f64, f64)>() > budget() {
            return Err(too_big(&format!("{points_read}+ points don't fit")));
        }
        let rows = (0..batch.num_rows()).map(|row| (row, (xs.value(row), ys.value(row))));
        match names {
            None => all.extend(rows.map(|(_, point)| point)),
            Some(names) => {
                for (row, point) in rows {
                    match groups.get_mut(names.value(row)) {
                        Some(values) => values.push(point),
                        None => {
                            groups.insert(names.value(row).to_string(), vec![point]);
                        }
                    }
                }
            }
        }
    }
    let mut points = RaggedVec::new();
    let mut names = Vec::new();
    if group.is_none() {
        points.push(all);
    }
    for (name, values) in groups {
        names.push(name);
        points.push(values);
    }
    Ok(PlotData::Scatter {
        points,
        x,
        y,
        groups: group.map(|_| names),
    })
}
