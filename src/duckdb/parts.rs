use crate::{
    AppResult,
    duckdb::{
        DataSource, DuckDB, Finished, Ingest, IngestOptions, Load,
        ingester::progress::{Deferred, Parts},
        load::{Probe, probe},
    },
    misc::workers,
};
use anyhow::anyhow;
use std::{
    path::PathBuf,
    sync::{Arc, atomic::AtomicBool},
};
use tokio::sync::oneshot::{self, Receiver};

type Schema = Vec<(String, String)>;
type Members = Vec<(String, String)>;

pub(super) struct PartsProbing {
    sources: Vec<DataSource>,
    options: IngestOptions,
    max_rows: Option<usize>,
    progress: Arc<Deferred>,
    result: Vec<Receiver<(String, Probe)>>,
}

pub(super) struct Group {
    pub(super) id: usize,
    pub(super) source: PathBuf,
    pub(super) limit: Option<usize>,
    pub(super) parts: Vec<(String, Option<String>)>,
    pub(super) waiting: usize,
    pub(super) error: Option<String>,
    pub(super) reported: bool,
}

fn source_path(sources: &[DataSource]) -> PathBuf {
    sources
        .first()
        .map(|source| PathBuf::from(source.display_path().as_ref()))
        .unwrap_or_default()
}

impl DuckDB {
    pub fn load_parts(
        &mut self,
        sources: &[DataSource],
        options: &IngestOptions,
        max_rows: Option<usize>,
    ) -> AppResult<Load> {
        let urls = sources
            .iter()
            .filter_map(|source| match source {
                DataSource::Url(url) if !self.probed.contains_key(url.as_str()) => {
                    Some(url.clone())
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        if !urls.is_empty() {
            let progress = Arc::new(Deferred::default());
            let result = urls
                .into_iter()
                .map(|url| {
                    let (sender, result) = oneshot::channel();
                    workers::spawn(move || {
                        let _ = sender.send((url.to_string(), probe(&url)));
                    });
                    result
                })
                .collect();
            self.parts_probing.push(PartsProbing {
                sources: sources.to_vec(),
                options: options.clone(),
                max_rows,
                progress: progress.clone(),
                result,
            });
            return Ok(Load::Ingesting(Ingest {
                source: source_path(sources),
                progress,
                file: None,
                complete: Arc::new(AtomicBool::new(true)),
            }));
        }
        let loaded = self.load_probed_parts(sources, options, max_rows);
        self.probed.clear();
        loaded
    }
    pub(super) fn poll_parts_probes(&mut self, finished: &mut Finished) {
        let mut i = 0;
        while i < self.parts_probing.len() {
            if self.parts_probing[i]
                .result
                .iter()
                .any(|result| result.is_empty() && !result.is_terminated())
            {
                i += 1;
                continue;
            }
            let mut probing = self.parts_probing.swap_remove(i);
            let probes = probing
                .result
                .iter_mut()
                .filter_map(|result| result.try_recv().ok())
                .collect::<Vec<_>>();
            for (url, probe) in probes {
                self.probed.insert(url, probe);
            }
            for source in &probing.sources {
                if let DataSource::Url(url) = source {
                    self.probed.entry(url.to_string()).or_default();
                }
            }
            let source = source_path(&probing.sources);
            match self.load_parts(&probing.sources, &probing.options, probing.max_rows) {
                Ok(Load::Ingesting(ingest)) => probing.progress.set(ingest.progress),
                Ok(Load::Ready(names)) => finished.tables.push((source, Ok(names))),
                Err(e) => finished.tables.push((source, Err(e))),
            }
        }
    }
    fn load_probed_parts(
        &mut self,
        sources: &[DataSource],
        options: &IngestOptions,
        max_rows: Option<usize>,
    ) -> AppResult<Load> {
        let id = self.groups.last().map(|group| group.id + 1).unwrap_or(0);
        self.groups.push(Group {
            id,
            source: source_path(sources),
            limit: max_rows,
            parts: Vec::new(),
            waiting: 0,
            error: None,
            reported: true,
        });
        self.part_of = Some(id);
        let mut running = Vec::new();
        let mut failed = None;
        for source in sources {
            let loaded = match source {
                DataSource::Stdin => self.load_stdin(None, options, None),
                DataSource::File(path) => self.load(path, None, options, None),
                DataSource::Url(url) => self.load_url(url, None, options, None),
            };
            match loaded {
                Ok(Load::Ingesting(ingest)) => running.push(ingest.progress),
                Ok(Load::Ready(_)) => {}
                Err(e) => {
                    failed = Some(e);
                    break;
                }
            }
        }
        self.part_of = None;
        let index = self.groups.len() - 1;
        if let Some(e) = failed {
            self.groups[index].error = Some(e.to_string());
            self.groups[index].reported = false;
            return Err(e);
        }
        if self.groups[index].waiting == 0 {
            let group = self.groups.remove(index);
            return self.complete_group(&group).map(Load::Ready);
        }
        Ok(Load::Ingesting(Ingest {
            source: self.groups[index].source.clone(),
            progress: Arc::new(Parts {
                done: self.groups[index].parts.len() - running.len(),
                parts: running,
            }),
            file: None,
            complete: Arc::new(AtomicBool::new(true)),
        }))
    }
    pub(super) fn join(&mut self, alias: &str, name: Option<&str>, running: bool) -> bool {
        let Some(group) = self
            .part_of
            .and_then(|id| self.groups.iter_mut().find(|group| group.id == id))
        else {
            return false;
        };
        group
            .parts
            .push((alias.to_string(), name.map(String::from)));
        group.waiting += running as usize;
        true
    }
    pub(super) fn complete_group(&mut self, group: &Group) -> AppResult<Vec<String>> {
        let names = self.union_views(group);
        for (alias, _) in &group.parts {
            self.session.unpin(alias);
        }
        names
    }
    fn union_views(&mut self, group: &Group) -> AppResult<Vec<String>> {
        if let Some(error) = &group.error {
            return Err(anyhow!("{error}"));
        }
        let mut sets: Vec<(Schema, String, Members)> = Vec::new();
        for (alias, name) in &group.parts {
            for table in self.session.tables(alias)? {
                let schema = self.session.columns(alias, &table)?;
                let view = match name {
                    Some(name) if table == "data" => name.clone(),
                    _ => table.clone(),
                };
                let member = (alias.clone(), table);
                match sets.iter_mut().find(|(columns, _, _)| *columns == schema) {
                    Some((_, _, members)) => members.push(member),
                    None => sets.push((schema, view, vec![member])),
                }
            }
        }
        if sets.is_empty() {
            return Err(anyhow!("No tables were loaded"));
        }
        sets.iter()
            .map(|(_, view, members)| self.session.create_union(view, members, group.limit))
            .collect()
    }
}
