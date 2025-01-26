use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

use itertools::Itertools;

pub struct History {
    path: Option<PathBuf>,
    start_len: usize,
    history: Vec<String>,
}

impl History {
    pub fn from_file(path: PathBuf) -> Self {
        let history = fs::read_to_string(path.as_path()).map_or(Default::default(), |string| {
            string
                .lines()
                .map(str::trim)
                .map(str::to_owned)
                .collect_vec()
        });
        Self {
            path: path.into(),
            start_len: history.len(),
            history,
        }
    }

    pub fn in_memory() -> Self {
        Self {
            path: None,
            start_len: 0,
            history: Default::default(),
        }
    }

    pub fn push(&mut self, cmd: String) {
        if self.history.last().map_or(true, |last| last != &cmd) {
            self.history.push(cmd);
        }
    }

    pub fn get(&self, idx: usize) -> Option<&String> {
        let idx = self.history.len().saturating_sub(1).saturating_sub(idx);
        self.history.get(idx)
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.history.iter().rev()
    }
}

impl Drop for History {
    fn drop(&mut self) {
        if let Some(path) = self.path.take() {
            if let Ok(mut file) = OpenOptions::new()
                .read(false)
                .write(true)
                .append(true)
                .create(true)
                .open(path)
            {
                for line in self.history.drain(..).skip(self.start_len) {
                    let _ = write!(file, "{}\n", line);
                }
            }
        }
    }
}
