use anyhow::anyhow;

use crate::{
    AppResult,
    sql::{
        db::{Database, Value, ValueRef},
        ops::{
            reply::{Receiver, channel},
            util::quote_ident,
        },
        worker::submit,
    },
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ScatterPlot {
    groups: Vec<Group>,
}

impl ScatterPlot {
    pub fn new(groups: Vec<Group>) -> Self {
        Self { groups }
    }

    pub fn groups(&self) -> &[Group] {
        &self.groups
    }

    pub fn len(&self) -> usize {
        self.groups.len()
    }

    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    key: Option<Value>,
    points: Vec<(f64, f64)>,
}

impl Group {
    pub fn key(&self) -> Option<&Value> {
        self.key.as_ref()
    }

    pub fn points(&self) -> &[(f64, f64)] {
        &self.points
    }
}

#[derive(Debug, Clone)]
pub struct Fetcher {
    sql: String,
    x: String,
    y: String,
    group: Option<String>,
}

impl Fetcher {
    pub fn new(sql: impl Into<String>, x: impl Into<String>, y: impl Into<String>) -> Self {
        Self {
            sql: sql.into(),
            x: x.into(),
            y: y.into(),
            group: None,
        }
    }

    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    pub fn run(&self) -> Receiver<ScatterPlot> {
        let this = self.clone();
        let (send, recv) = channel();
        submit(move |db| send.send(this.fetch(db)));
        recv
    }

    fn fetch(&self, db: &Database) -> AppResult<ScatterPlot> {
        let sql = self.sql.trim().trim_end_matches(';');
        let x = quote_ident(&self.x);
        let y = quote_ident(&self.y);
        match &self.group {
            Some(group) => {
                let group = quote_ident(group);
                let mut stmt = db.prepare(&format!(
                    "SELECT {x}::DOUBLE, {y}::DOUBLE, {group} FROM ({sql}) \
                     WHERE isfinite({x}::DOUBLE) AND isfinite({y}::DOUBLE) \
                     ORDER BY {group} NULLS LAST"
                ))?;
                let mut rows = stmt.query([])?;
                let mut groups: Vec<Group> = Vec::new();
                while let Some(row) = rows.next()? {
                    let point = point(row.get_ref(0)?, row.get_ref(1)?)?;
                    let key = Value::from(row.get_ref(2)?);
                    match groups.last_mut() {
                        Some(last) if last.key.as_ref() == Some(&key) => last.points.push(point),
                        _ => groups.push(Group {
                            key: Some(key),
                            points: vec![point],
                        }),
                    }
                }
                Ok(ScatterPlot::new(groups))
            }
            None => {
                let mut stmt = db.prepare(&format!(
                    "SELECT {x}::DOUBLE, {y}::DOUBLE FROM ({sql}) \
                     WHERE isfinite({x}::DOUBLE) AND isfinite({y}::DOUBLE)"
                ))?;
                let mut rows = stmt.query([])?;
                let mut points = Vec::new();
                while let Some(row) = rows.next()? {
                    points.push(point(row.get_ref(0)?, row.get_ref(1)?)?);
                }
                Ok(ScatterPlot::new(vec![Group { key: None, points }]))
            }
        }
    }
}

fn point(x: ValueRef, y: ValueRef) -> AppResult<(f64, f64)> {
    match (x, y) {
        (ValueRef::Double(x), ValueRef::Double(y)) => Ok((x, y)),
        other => Err(anyhow!("unexpected scatter plot point: {other:?}")),
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;
    use crate::sql::{db::DatabaseConfig, ops::Next};

    fn db() -> Database {
        DatabaseConfig::default().open_in_memory().unwrap()
    }

    fn wait<T>(recv: &Receiver<T>) -> AppResult<T> {
        let deadline = Instant::now() + Duration::from_secs(30);
        loop {
            match recv.recv() {
                Next::Value(value) => return value,
                Next::Done => panic!("sender dropped without sending a value"),
                Next::Empty if Instant::now() > deadline => panic!("timed out waiting for a value"),
                Next::Empty => std::thread::sleep(Duration::from_millis(5)),
            }
        }
    }

    fn sorted(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
        let mut points = points.to_vec();
        points.sort_by(|a, b| a.partial_cmp(b).unwrap());
        points
    }

    #[test]
    fn ungrouped_points_are_one_group_without_a_key() {
        let plot = Fetcher::new("SELECT range AS x, range * 2 AS y FROM range(3)", "x", "y")
            .fetch(&db())
            .unwrap();
        assert_eq!(plot.len(), 1);
        let group = &plot.groups()[0];
        assert_eq!(group.key(), None);
        assert_eq!(sorted(group.points()), [(0.0, 0.0), (1.0, 2.0), (2.0, 4.0)]);
    }

    #[test]
    fn nulls_and_non_finite_values_are_skipped() {
        let plot = Fetcher::new(
            "SELECT * FROM (VALUES (1.0, 1.0), (NULL, 2.0), (3.0, NULL), \
             ('nan'::DOUBLE, 4.0), (5.0, 'inf'::DOUBLE), (6.0, 6.0)) v(x, y)",
            "x",
            "y",
        )
        .fetch(&db())
        .unwrap();
        assert_eq!(sorted(plot.groups()[0].points()), [(1.0, 1.0), (6.0, 6.0)]);
    }

    #[test]
    fn empty_result_is_one_empty_group() {
        let plot = Fetcher::new("SELECT 1 AS x, 2 AS y WHERE false", "x", "y")
            .fetch(&db())
            .unwrap();
        assert_eq!(plot.len(), 1);
        assert!(plot.groups()[0].points().is_empty());
    }

    #[test]
    fn points_are_grouped_by_value_in_order() {
        let plot = Fetcher::new(
            "SELECT * FROM (VALUES (1, 1, 'b'), (2, 2, 'a'), (3, 3, 'b'), (4, 4, NULL)) v(x, y, g)",
            "x",
            "y",
        )
        .with_group("g")
        .fetch(&db())
        .unwrap();
        let keys = plot
            .groups()
            .iter()
            .map(|group| group.key().cloned())
            .collect::<Vec<_>>();
        assert_eq!(
            keys,
            [
                Some(Value::Text("a".into())),
                Some(Value::Text("b".into())),
                Some(Value::Null),
            ]
        );
        assert_eq!(plot.groups()[0].points(), [(2.0, 2.0)]);
        assert_eq!(sorted(plot.groups()[1].points()), [(1.0, 1.0), (3.0, 3.0)]);
        assert_eq!(plot.groups()[2].points(), [(4.0, 4.0)]);
    }

    #[test]
    fn grouped_empty_result_has_no_groups() {
        let plot = Fetcher::new("SELECT 1 AS x, 2 AS y, 'a' AS g WHERE false", "x", "y")
            .with_group("g")
            .fetch(&db())
            .unwrap();
        assert!(plot.is_empty());
    }

    #[test]
    fn quoted_column_names_work() {
        let plot = Fetcher::new(
            "SELECT 1 AS \"x x\", 2 AS \"y y\", 3 AS \"g g\"",
            "x x",
            "y y",
        )
        .with_group("g g")
        .fetch(&db())
        .unwrap();
        assert_eq!(plot.groups()[0].key(), Some(&Value::Int(3)));
        assert_eq!(plot.groups()[0].points(), [(1.0, 2.0)]);
    }

    #[test]
    fn missing_column_fails() {
        assert!(
            Fetcher::new("SELECT 1 AS x", "x", "missing")
                .fetch(&db())
                .is_err()
        );
    }

    #[test]
    fn run_returns_the_plot_through_the_worker() {
        let plot = wait(&Fetcher::new("SELECT 1 AS x, 2 AS y", "x", "y").run()).unwrap();
        assert_eq!(plot.groups()[0].points(), [(1.0, 2.0)]);
    }
}
