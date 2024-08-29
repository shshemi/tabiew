use crate::AppResult;

use super::{tabular::Tabular, Styler};

#[derive(Debug, Default)]
pub struct Tabs<Theme> {
    tabulars: Vec<Tabular<Theme>>,
    idx: usize,
}

impl<Theme: Styler> Tabs<Theme> {
    pub fn add(&mut self, tabular: Tabular<Theme>) -> AppResult<()> {
        self.tabulars.push(tabular);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.tabulars.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn idx(&self) -> usize {
        self.idx
    }

    pub fn selected(&self) -> Option<&Tabular<Theme>> {
        self.tabulars.get(self.idx)
    }

    pub fn selected_mut(&mut self) -> Option<&mut Tabular<Theme>> {
        self.tabulars.get_mut(self.idx)
    }

    pub fn remove(&mut self, idx: usize) -> AppResult<()> {
        self.validate_index(idx)?;
        self.tabulars.remove(idx);
        self.saturating_select(self.idx.saturating_sub(1))
    }

    pub fn remove_selected(&mut self) -> AppResult<()> {
        self.remove(self.idx)
    }

    pub fn saturating_select(&mut self, idx: usize) -> AppResult<()> {
        self.idx = idx.min(self.tabulars.len().saturating_sub(1));
        Ok(())
    }

    pub fn select(&mut self, idx: usize) -> AppResult<()> {
        self.validate_index(idx)?;
        self.idx = idx;
        Ok(())
    }

    pub fn select_next(&mut self) -> AppResult<()> {
        self.saturating_select(self.idx.saturating_add(1))
    }

    pub fn select_prev(&mut self) -> AppResult<()> {
        self.saturating_select(self.idx.saturating_sub(1))
    }

    pub fn select_first(&mut self) -> AppResult<()> {
        self.saturating_select(0)
    }

    pub fn select_last(&mut self) -> AppResult<()> {
        self.saturating_select(usize::MAX)
    }

    fn validate_index(&self, idx: usize) -> AppResult<()> {
        if self.tabulars.is_empty() {
            Err("no tab is currently available".into())
        } else if idx < self.tabulars.len() {
            Ok(())
        } else {
            Err(format!(
                "invalid tab index, valid index range is between 0 and {}",
                self.tabulars.len()
            )
            .into())
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Tabular<Theme>> {
        self.tabulars.iter()
    }
}

impl<Theme> FromIterator<Tabular<Theme>> for Tabs<Theme> {
    fn from_iter<T: IntoIterator<Item = Tabular<Theme>>>(iter: T) -> Self {
        Self {
            tabulars: iter.into_iter().collect(),
            idx: 0,
        }
    }
}
