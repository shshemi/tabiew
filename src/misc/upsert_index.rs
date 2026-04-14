//! Composite-key upsert index for streaming data.
//!
//! An `UpsertIndex` tracks a `key_bytes -> live_row_idx` map so that each
//! incoming batch can be partitioned into (updates to existing keys) and
//! (inserts of new keys). Inserts append to the live frame via `vstack`;
//! updates are applied column-by-column via `try_apply_at_idx`, which is
//! order-preserving by construction — the key→row-index map stays valid.
//!
//! The live frame must only be mutated via `apply_batch` (or equivalent
//! in-order appends and in-place column writes) so the row-index map stays
//! in sync. Row deletion or reordering would invalidate the map.

use std::collections::HashMap;

use polars::prelude::*;

use crate::AppResult;

/// Per-batch statistics returned by `UpsertIndex::apply_batch`.
#[derive(Debug, Default, Clone)]
pub struct UpsertStats {
    pub inserted: usize,
    pub updated: usize,
    /// Live-frame row indices of newly inserted rows.
    pub inserted_rows: Vec<usize>,
    /// (live_row_idx, col_idx) pairs for cells updated in place.
    pub updated_cells: Vec<(usize, usize)>,
}

#[derive(Debug)]
pub struct UpsertIndex {
    key_cols: Vec<usize>,
    key_dtypes: Vec<DataType>,
    map: HashMap<Vec<u8>, usize>,
    locked: bool,
}

impl UpsertIndex {
    pub fn new(key_cols: Vec<usize>) -> Self {
        Self {
            key_cols,
            key_dtypes: Vec::new(),
            map: HashMap::new(),
            locked: false,
        }
    }

    pub fn key_cols(&self) -> &[usize] {
        &self.key_cols
    }

    /// Apply a batch of rows as upserts against the live frame. On first
    /// call, the key column indexes are validated against the batch schema
    /// and their dtypes are locked.
    pub fn apply_batch(
        &mut self,
        live: &mut DataFrame,
        batch: DataFrame,
    ) -> AppResult<UpsertStats> {
        if !self.locked {
            self.lock_dtypes(batch.schema())?;
            self.locked = true;
        }

        // Compute one key per batch row.
        let batch_keys = self.compute_keys(&batch)?;

        // Within a single batch, only the last occurrence of a given key
        // wins; earlier occurrences are dropped. We pre-compute the index
        // of the last occurrence per key.
        let mut last_pos_for_key: HashMap<&[u8], usize> = HashMap::new();
        for (i, key) in batch_keys.iter().enumerate() {
            last_pos_for_key.insert(key.as_slice(), i);
        }

        // Partition in batch order, skipping rows dominated by a later one.
        let mut updates: Vec<(usize, usize)> = Vec::new(); // (live_row_idx, batch_row_idx)
        let mut inserts: Vec<usize> = Vec::new(); // batch_row_idx
        for (i, key) in batch_keys.iter().enumerate() {
            if last_pos_for_key.get(key.as_slice()) != Some(&i) {
                continue;
            }
            if let Some(&live_idx) = self.map.get(key.as_slice()) {
                updates.push((live_idx, i));
            } else {
                inserts.push(i);
            }
        }

        // Apply inserts first: vstack, then extend the map.
        let mut inserted_rows = Vec::new();
        if !inserts.is_empty() {
            let base_height = live.height();
            let insert_idx: Vec<IdxSize> = inserts.iter().map(|&i| i as IdxSize).collect();
            let insert_ca = IdxCa::from_vec("_take_idx".into(), insert_idx);
            let insert_df = batch.take(&insert_ca)?;
            if live.width() == 0 {
                *live = insert_df;
            } else {
                live.vstack_mut_owned(insert_df)?;
            }
            for (offset, &batch_row) in inserts.iter().enumerate() {
                let row_idx = base_height + offset;
                self.map
                    .insert(batch_keys[batch_row].clone(), row_idx);
                inserted_rows.push(row_idx);
            }
        }

        // Apply updates: per column, build a replacement series that keeps
        // original values everywhere except the updated live-row indices.
        // Track which cells actually changed value (skip key columns and
        // cells where old == new).
        let mut updated_cells = Vec::new();
        let width = live.width();
        if !updates.is_empty() {
            // Sort by live_row_idx so a single forward pass over the column
            // is enough.
            updates.sort_by_key(|(live_idx, _)| *live_idx);
            for col_idx in 0..width {
                if self.key_cols.contains(&col_idx) {
                    continue;
                }
                let src_col = batch.columns()[col_idx].clone();
                // Compare old vs new before overwriting.
                let live_col = &live.columns()[col_idx];
                for &(live_idx, batch_idx) in &updates {
                    let old_val = live_col.get(live_idx).unwrap();
                    let new_val = src_col.get(batch_idx).unwrap();
                    if old_val != new_val {
                        updated_cells.push((live_idx, col_idx));
                    }
                }
            }
            // Now apply the actual updates.
            for col_idx in 0..width {
                let src_col = batch.columns()[col_idx].clone();
                live.try_apply_at_idx(col_idx, |col| {
                    build_updated_series(col, &src_col, &updates)
                })?;
            }
        }

        Ok(UpsertStats {
            inserted: inserts.len(),
            updated: updates.len(),
            inserted_rows,
            updated_cells,
        })
    }

    fn lock_dtypes(&mut self, schema: &Schema) -> AppResult<()> {
        let n = schema.len();
        for &k in &self.key_cols {
            if k >= n {
                return Err(anyhow::anyhow!(
                    "--key index {k} out of range; data has {n} columns"
                ));
            }
        }
        self.key_dtypes = self
            .key_cols
            .iter()
            .map(|&i| schema.get_at_index(i).unwrap().1.clone())
            .collect();
        Ok(())
    }

    fn compute_keys(&self, df: &DataFrame) -> AppResult<Vec<Vec<u8>>> {
        let height = df.height();
        let mut out = Vec::with_capacity(height);
        let cols = df.columns();
        let key_series: Vec<Series> = self
            .key_cols
            .iter()
            .map(|&i| cols[i].as_materialized_series().clone())
            .collect();
        // Validate dtypes haven't drifted.
        for (s, locked) in key_series.iter().zip(self.key_dtypes.iter()) {
            if s.dtype() != locked {
                return Err(anyhow::anyhow!(
                    "key column `{}` dtype changed from {:?} to {:?} between batches",
                    s.name(),
                    locked,
                    s.dtype()
                ));
            }
        }
        for row in 0..height {
            let mut key = Vec::new();
            for s in &key_series {
                let val = s.get(row)?;
                encode_any_value(&val, &mut key);
            }
            out.push(key);
        }
        Ok(out)
    }
}

/// Build a new series where positions listed in `updates` are replaced with
/// values pulled from `src_col` at their paired batch-row indexes. `updates`
/// must be sorted ascending by live-row index.
fn build_updated_series(
    col: &Column,
    src_col: &Column,
    updates: &[(usize, usize)],
) -> PolarsResult<Series> {
    let s = col.as_materialized_series();
    let src = src_col.as_materialized_series();
    let n = s.len();
    let dtype = s.dtype().clone();
    let name = s.name().clone();
    let mut new_values: Vec<AnyValue<'_>> = Vec::with_capacity(n);
    let mut u_ptr = 0;
    for i in 0..n {
        if u_ptr < updates.len() && updates[u_ptr].0 == i {
            let batch_row = updates[u_ptr].1;
            new_values.push(src.get(batch_row)?);
            u_ptr += 1;
        } else {
            new_values.push(s.get(i)?);
        }
    }
    Series::from_any_values_and_dtype(name, &new_values, &dtype, true)
}

/// Length-prefixed, type-tagged byte encoding of an `AnyValue`. The encoding
/// is injective across the supported dtypes: e.g. ("ab", "c") and ("a", "bc")
/// encode to distinct byte sequences because every variable-length payload is
/// preceded by its length.
fn encode_any_value(v: &AnyValue<'_>, out: &mut Vec<u8>) {
    const TAG_NULL: u8 = 0;
    const TAG_I64: u8 = 1;
    const TAG_U64: u8 = 2;
    const TAG_F64: u8 = 3;
    const TAG_BOOL: u8 = 4;
    const TAG_STR: u8 = 5;
    const TAG_BIN: u8 = 6;
    const TAG_OTHER: u8 = 7;

    fn push_len_prefixed(out: &mut Vec<u8>, tag: u8, bytes: &[u8]) {
        out.push(tag);
        out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(bytes);
    }

    match v {
        AnyValue::Null => out.push(TAG_NULL),
        AnyValue::Boolean(b) => {
            out.push(TAG_BOOL);
            out.push(u8::from(*b));
        }
        AnyValue::Int8(n) => {
            out.push(TAG_I64);
            out.extend_from_slice(&(i64::from(*n)).to_le_bytes());
        }
        AnyValue::Int16(n) => {
            out.push(TAG_I64);
            out.extend_from_slice(&(i64::from(*n)).to_le_bytes());
        }
        AnyValue::Int32(n) => {
            out.push(TAG_I64);
            out.extend_from_slice(&(i64::from(*n)).to_le_bytes());
        }
        AnyValue::Int64(n) => {
            out.push(TAG_I64);
            out.extend_from_slice(&n.to_le_bytes());
        }
        AnyValue::UInt8(n) => {
            out.push(TAG_U64);
            out.extend_from_slice(&(u64::from(*n)).to_le_bytes());
        }
        AnyValue::UInt16(n) => {
            out.push(TAG_U64);
            out.extend_from_slice(&(u64::from(*n)).to_le_bytes());
        }
        AnyValue::UInt32(n) => {
            out.push(TAG_U64);
            out.extend_from_slice(&(u64::from(*n)).to_le_bytes());
        }
        AnyValue::UInt64(n) => {
            out.push(TAG_U64);
            out.extend_from_slice(&n.to_le_bytes());
        }
        AnyValue::Float32(f) => {
            out.push(TAG_F64);
            out.extend_from_slice(&(f64::from(*f)).to_le_bytes());
        }
        AnyValue::Float64(f) => {
            out.push(TAG_F64);
            out.extend_from_slice(&f.to_le_bytes());
        }
        AnyValue::String(s) => push_len_prefixed(out, TAG_STR, s.as_bytes()),
        AnyValue::StringOwned(s) => push_len_prefixed(out, TAG_STR, s.as_str().as_bytes()),
        AnyValue::Binary(b) => push_len_prefixed(out, TAG_BIN, b),
        AnyValue::BinaryOwned(b) => push_len_prefixed(out, TAG_BIN, b),
        other => {
            let s = format!("{other:?}");
            push_len_prefixed(out, TAG_OTHER, s.as_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn df_i64_str(ids: &[i64], vs: &[&str]) -> DataFrame {
        let id = Column::new("id".into(), ids);
        let v = Column::new("v".into(), vs);
        DataFrame::new_infer_height(vec![id, v]).unwrap()
    }

    #[test]
    fn pure_inserts_append_in_order() {
        let mut live = DataFrame::empty();
        let mut idx = UpsertIndex::new(vec![0]);
        let batch = df_i64_str(&[1, 2, 3], &["a", "b", "c"]);
        let stats = idx.apply_batch(&mut live, batch).unwrap();
        assert_eq!(stats.inserted, 3);
        assert_eq!(stats.updated, 0);
        assert_eq!(live.height(), 3);
        let v_col = live.column("v").unwrap().str().unwrap();
        let vs: Vec<_> = v_col.into_iter().collect();
        assert_eq!(vs, vec![Some("a"), Some("b"), Some("c")]);
    }

    #[test]
    fn update_preserves_row_position() {
        let mut live = DataFrame::empty();
        let mut idx = UpsertIndex::new(vec![0]);
        idx.apply_batch(&mut live, df_i64_str(&[1, 2, 3], &["a", "b", "c"]))
            .unwrap();
        // Update id=2 to "Z" — it should remain in position 1.
        let stats = idx
            .apply_batch(&mut live, df_i64_str(&[2], &["Z"]))
            .unwrap();
        assert_eq!(stats.inserted, 0);
        assert_eq!(stats.updated, 1);
        assert_eq!(live.height(), 3);
        let vs: Vec<_> = live
            .column("v")
            .unwrap()
            .str()
            .unwrap()
            .into_iter()
            .collect();
        assert_eq!(vs, vec![Some("a"), Some("Z"), Some("c")]);
    }

    #[test]
    fn last_write_wins_within_batch() {
        let mut live = DataFrame::empty();
        let mut idx = UpsertIndex::new(vec![0]);
        // Same key appears twice in one batch — only the last value survives.
        let stats = idx
            .apply_batch(&mut live, df_i64_str(&[1, 1], &["first", "last"]))
            .unwrap();
        assert_eq!(stats.inserted, 1);
        assert_eq!(stats.updated, 0);
        assert_eq!(live.height(), 1);
        let vs: Vec<_> = live
            .column("v")
            .unwrap()
            .str()
            .unwrap()
            .into_iter()
            .collect();
        assert_eq!(vs, vec![Some("last")]);
    }

    #[test]
    fn batch_mixing_updates_and_inserts() {
        let mut live = DataFrame::empty();
        let mut idx = UpsertIndex::new(vec![0]);
        idx.apply_batch(&mut live, df_i64_str(&[1, 2], &["a", "b"]))
            .unwrap();
        // Batch: update id=1, insert id=3, update id=2.
        let stats = idx
            .apply_batch(&mut live, df_i64_str(&[1, 3, 2], &["A", "c", "B"]))
            .unwrap();
        assert_eq!(stats.inserted, 1);
        assert_eq!(stats.updated, 2);
        assert_eq!(live.height(), 3);
        let vs: Vec<_> = live
            .column("v")
            .unwrap()
            .str()
            .unwrap()
            .into_iter()
            .collect();
        assert_eq!(vs, vec![Some("A"), Some("B"), Some("c")]);
    }

    #[test]
    fn composite_key_treats_only_full_match_as_update() {
        let mut live = DataFrame::empty();
        let mut idx = UpsertIndex::new(vec![0, 1]);
        let batch = DataFrame::new_infer_height(vec![
            Column::new("a".into(), &[1i64, 1, 1]),
            Column::new("b".into(), &[1i64, 2, 1]),
            Column::new("v".into(), &["x", "y", "z"]),
        ])
        .unwrap();
        // (1,1)=x then (1,2)=y then (1,1)=z — last (1,1) wins, (1,2) inserts.
        let stats = idx.apply_batch(&mut live, batch).unwrap();
        assert_eq!(stats.inserted, 2);
        assert_eq!(stats.updated, 0);
        assert_eq!(live.height(), 2);
        let vs: Vec<_> = live
            .column("v")
            .unwrap()
            .str()
            .unwrap()
            .into_iter()
            .collect();
        // Inserts are emitted in batch order, skipping rows dominated by a
        // later occurrence of the same key. Batch row 0 (1,1)=x is dominated
        // by row 2 (1,1)=z, so row 1 (1,2)=y is emitted first, then row 2.
        assert_eq!(vs, vec![Some("y"), Some("z")]);
    }

    #[test]
    fn many_updates_to_same_key_collapse_to_one_row() {
        let mut live = DataFrame::empty();
        let mut idx = UpsertIndex::new(vec![0]);
        idx.apply_batch(&mut live, df_i64_str(&[1], &["initial"]))
            .unwrap();
        for n in 0..100 {
            let label = format!("v{n}");
            idx.apply_batch(&mut live, df_i64_str(&[1], &[label.as_str()]))
                .unwrap();
        }
        assert_eq!(live.height(), 1);
        let last = live.column("v").unwrap().str().unwrap().get(0);
        assert_eq!(last, Some("v99"));
    }

    #[test]
    fn key_index_out_of_range_errors() {
        let mut live = DataFrame::empty();
        let mut idx = UpsertIndex::new(vec![5]);
        let err = idx
            .apply_batch(&mut live, df_i64_str(&[1, 2], &["a", "b"]))
            .unwrap_err();
        assert!(err.to_string().contains("out of range"));
    }

    #[test]
    fn key_encoding_distinguishes_overlap() {
        let mut a = Vec::new();
        let mut b = Vec::new();
        encode_any_value(&AnyValue::String("ab"), &mut a);
        encode_any_value(&AnyValue::String("c"), &mut a);
        encode_any_value(&AnyValue::String("a"), &mut b);
        encode_any_value(&AnyValue::String("bc"), &mut b);
        assert_ne!(a, b);
    }

    #[test]
    fn stats_contain_inserted_row_indices() {
        let mut live = DataFrame::empty();
        let mut idx = UpsertIndex::new(vec![0]);
        let stats = idx
            .apply_batch(&mut live, df_i64_str(&[10, 20, 30], &["a", "b", "c"]))
            .unwrap();
        assert_eq!(stats.inserted_rows, vec![0, 1, 2]);
        assert!(stats.updated_cells.is_empty());
    }

    #[test]
    fn stats_contain_updated_cell_coordinates() {
        let mut live = DataFrame::empty();
        let mut idx = UpsertIndex::new(vec![0]);
        idx.apply_batch(&mut live, df_i64_str(&[1, 2, 3], &["a", "b", "c"]))
            .unwrap();
        // Update id=2 → should produce updated_cells for row 1, non-key columns only.
        let stats = idx
            .apply_batch(&mut live, df_i64_str(&[2], &["Z"]))
            .unwrap();
        assert_eq!(stats.inserted_rows, Vec::<usize>::new());
        // Column 0 is the key — only column 1 (v) is marked as updated.
        assert_eq!(stats.updated_cells, vec![(1, 1)]);
    }

    #[test]
    fn stats_mixed_inserts_and_updates() {
        let mut live = DataFrame::empty();
        let mut idx = UpsertIndex::new(vec![0]);
        idx.apply_batch(&mut live, df_i64_str(&[1, 2], &["a", "b"]))
            .unwrap();
        // id=2 updates, id=3 inserts.
        let stats = idx
            .apply_batch(&mut live, df_i64_str(&[2, 3], &["B", "c"]))
            .unwrap();
        assert_eq!(stats.inserted, 1);
        assert_eq!(stats.updated, 1);
        // Insert at row 2 (after existing 2 rows).
        assert_eq!(stats.inserted_rows, vec![2]);
        // Update at row 1 (id=2), non-key column only.
        assert_eq!(stats.updated_cells, vec![(1, 1)]);
    }

    #[test]
    fn unchanged_values_not_in_updated_cells() {
        // Simulates stream-csv.sh: key=0, region (col 1) stays same,
        // only col 2 (v) changes.
        let id = Column::new("id".into(), &[1i64, 2]);
        let region = Column::new("region".into(), &["us", "eu"]);
        let v = Column::new("v".into(), &["old1", "old2"]);
        let initial = DataFrame::new_infer_height(vec![id, region, v]).unwrap();

        let mut live = DataFrame::empty();
        let mut idx = UpsertIndex::new(vec![0]);
        idx.apply_batch(&mut live, initial).unwrap();

        // Update id=1: region stays "us", v changes to "new1".
        let id2 = Column::new("id".into(), &[1i64]);
        let region2 = Column::new("region".into(), &["us"]);
        let v2 = Column::new("v".into(), &["new1"]);
        let batch = DataFrame::new_infer_height(vec![id2, region2, v2]).unwrap();
        let stats = idx.apply_batch(&mut live, batch).unwrap();

        assert_eq!(stats.updated, 1);
        // Only col 2 (v) changed — col 0 is key, col 1 (region) unchanged.
        assert_eq!(stats.updated_cells, vec![(0, 2)]);
    }
}
