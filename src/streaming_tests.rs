//! End-to-end integration tests for the streaming + upsert pipeline.
//!
//! These tests cross module boundaries: bytes go into a real stream
//! reader, events come out of a real mpsc channel, and rows are applied
//! to a live DataFrame through a real UpsertIndex — mirroring what
//! `App::drain_stream` does at runtime, but without spinning up a TUI.

use std::io::Cursor;
use std::sync::mpsc::sync_channel;

use polars::frame::DataFrame;
use polars::prelude::Schema;

use crate::args::InferSchema;
use crate::misc::upsert_index::UpsertIndex;
use crate::reader::StreamEvent;
use crate::reader::csv::{CsvReaderOpts, run_csv_stream};
use crate::reader::fwf::{FwfReaderOpts, run_fwf_stream};
use crate::reader::json_line::run_jsonl_stream;
use crate::reader::logfmt::run_logfmt_stream;
use crate::reader::traits::StreamingConfig;

fn tight_config() -> StreamingConfig {
    // Small batches so multi-row inputs get chunked across several Batch
    // events — that's where schema-locking and upsert ordering matter.
    StreamingConfig {
        batch_rows: 2,
        batch_ms: 10_000,
    }
}

/// Drain every event into a (schema, batches, eof) tuple, panicking on
/// Error events.
fn collect(rx: std::sync::mpsc::Receiver<StreamEvent>) -> (Option<Schema>, Vec<DataFrame>, bool) {
    let mut schema = None;
    let mut batches = Vec::new();
    let mut eof = false;
    while let Ok(evt) = rx.recv() {
        match evt {
            StreamEvent::Schema { schema: s, .. } => schema = Some(s.as_ref().clone()),
            StreamEvent::Batch { rows, .. } => batches.push(rows),
            StreamEvent::Eof { .. } => eof = true,
            StreamEvent::Error { error, .. } => panic!("unexpected stream error: {error}"),
        }
    }
    (schema, batches, eof)
}

/// Apply every emitted batch to a live DataFrame via UpsertIndex. Returns
/// the live frame plus cumulative (inserted, updated) counters.
fn apply_batches(
    schema: &Schema,
    batches: Vec<DataFrame>,
    key_cols: Vec<usize>,
) -> (DataFrame, usize, usize) {
    let mut live = DataFrame::empty_with_schema(schema);
    let mut upsert = UpsertIndex::new(key_cols);
    let mut inserted = 0;
    let mut updated = 0;
    for b in batches {
        let stats = upsert.apply_batch(&mut live, b).expect("apply_batch");
        inserted += stats.inserted;
        updated += stats.updated;
    }
    (live, inserted, updated)
}

#[test]
fn jsonl_stream_with_key_upserts_rows_in_place() {
    // Two records for id=1 (second wins), one for id=2, another update to id=1.
    // Final state: id=1 → v="final", id=2 → v="b".
    let data =
        b"{\"id\":1,\"v\":\"first\"}\n{\"id\":1,\"v\":\"second\"}\n{\"id\":2,\"v\":\"b\"}\n{\"id\":1,\"v\":\"final\"}\n".to_vec();
    let (tx, rx) = sync_channel::<StreamEvent>(16);
    run_jsonl_stream(Cursor::new(data), "t", true, tight_config(), &tx);
    drop(tx);

    let (schema, batches, eof) = collect(rx);
    assert!(eof);
    let schema = schema.expect("schema emitted");
    assert_eq!(batches.len(), 2, "expected exactly two batches");

    let (live, inserted, updated) = apply_batches(&schema, batches, vec![0]);
    assert_eq!(live.height(), 2, "only two distinct keys");
    assert_eq!(inserted, 2);
    assert_eq!(updated, 1);

    let sorted = live.sort(["id"], Default::default()).unwrap();
    let vs: Vec<_> = sorted
        .column("v")
        .unwrap()
        .str()
        .unwrap()
        .into_iter()
        .collect();
    assert_eq!(vs, vec![Some("final"), Some("b")]);
}

#[test]
fn csv_stream_with_composite_key_treats_only_full_match_as_update() {
    // Composite key (col 0, col 1). Row (1,1) is superseded within the
    // input; (1,2) and (2,1) are distinct keys.
    let data = b"a,b,v\n1,1,x\n1,2,y\n2,1,z\n1,1,w\n".to_vec();
    let (tx, rx) = sync_channel::<StreamEvent>(16);
    run_csv_stream(
        Cursor::new(data),
        "t",
        &CsvReaderOpts {
            infer_schema: InferSchema::Safe,
            quote_char: '"',
            separator_char: ',',
            no_header: false,
            ignore_errors: true,
            truncate_ragged_lines: false,
        },
        tight_config(),
        &tx,
    );
    drop(tx);

    let (schema, batches, eof) = collect(rx);
    assert!(eof);
    let schema = schema.unwrap();
    let (live, inserted, updated) = apply_batches(&schema, batches, vec![0, 1]);
    assert_eq!(live.height(), 3);
    assert_eq!(inserted, 3);
    assert_eq!(updated, 1);

    // Check by (a,b)→v lookup rather than positional — CSV safe-infer keeps
    // everything as str, which is fine for a composite-key check.
    let mut seen = std::collections::HashMap::new();
    let a = live.column("a").unwrap().str().unwrap();
    let b = live.column("b").unwrap().str().unwrap();
    let v = live.column("v").unwrap().str().unwrap();
    for i in 0..live.height() {
        seen.insert(
            (a.get(i).unwrap().to_string(), b.get(i).unwrap().to_string()),
            v.get(i).unwrap().to_string(),
        );
    }
    assert_eq!(seen.get(&("1".into(), "1".into())), Some(&"w".to_string()));
    assert_eq!(seen.get(&("1".into(), "2".into())), Some(&"y".to_string()));
    assert_eq!(seen.get(&("2".into(), "1".into())), Some(&"z".to_string()));
}

#[test]
fn logfmt_stream_locks_schema_and_upserts_by_first_column() {
    // Batch 1 has columns a,b. Batch 2 adds c (dropped) and omits b for
    // one row (null-filled). Upsert on column "a".
    let data = b"a=1 b=x\na=2 b=y\na=1 b=X c=new\na=3\n".to_vec();
    let (tx, rx) = sync_channel::<StreamEvent>(16);
    run_logfmt_stream(Cursor::new(data), "t", tight_config(), &tx);
    drop(tx);

    let (schema, batches, eof) = collect(rx);
    assert!(eof);
    let schema = schema.expect("schema emitted");
    // Only a, b — `c` showed up after the schema was locked and must be dropped.
    let names: Vec<_> = schema.iter_names().map(|s| s.to_string()).collect();
    assert_eq!(names, vec!["a", "b"]);

    let (live, inserted, updated) = apply_batches(&schema, batches, vec![0]);
    assert_eq!(live.height(), 3, "distinct keys: 1, 2, 3");
    assert_eq!(inserted, 3);
    assert_eq!(updated, 1);

    // a=1 should now have b="X" (the later value), a=3 should have null b.
    let mut by_a = std::collections::HashMap::new();
    let a = live.column("a").unwrap().str().unwrap();
    let b = live.column("b").unwrap().str().unwrap();
    for i in 0..live.height() {
        by_a.insert(a.get(i).unwrap().to_string(), b.get(i).map(str::to_string));
    }
    assert_eq!(by_a.get("1"), Some(&Some("X".to_string())));
    assert_eq!(by_a.get("2"), Some(&Some("y".to_string())));
    assert_eq!(by_a.get("3"), Some(&None));
}

#[test]
fn fwf_stream_requires_widths_and_streams_with_schema_lock() {
    // Fixed widths: name=10, id=5. Headers on the first line.
    let data = b"name      id   \nalice     1    \nbob       2    \nalice     9    \n".to_vec();
    let (tx, rx) = sync_channel::<StreamEvent>(16);
    run_fwf_stream(
        Cursor::new(data),
        "t",
        &FwfReaderOpts {
            widths: vec![10, 5],
            has_header: true,
            separator_length: 0,
            flexible_width: true,
        },
        tight_config(),
        &tx,
    );
    drop(tx);

    let (schema, batches, eof) = collect(rx);
    assert!(eof);
    let schema = schema.expect("schema emitted");
    let (live, inserted, updated) = apply_batches(&schema, batches, vec![0]);
    // Keys: "alice", "bob". Second "alice" row is an update.
    assert_eq!(live.height(), 2);
    assert_eq!(inserted, 2);
    assert_eq!(updated, 1);

    // Column names come straight from the fixed-width header slice, so
    // they may carry trailing padding. Look them up positionally.
    let col_names: Vec<String> = live
        .get_column_names()
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let name = live.column(&col_names[0]).unwrap().str().unwrap();
    let id = live.column(&col_names[1]).unwrap().str().unwrap();
    let mut by_name = std::collections::HashMap::new();
    for i in 0..live.height() {
        by_name.insert(
            name.get(i).unwrap().to_string(),
            id.get(i).unwrap().to_string(),
        );
    }
    assert_eq!(by_name.get("alice"), Some(&"9".to_string()));
    assert_eq!(by_name.get("bob"), Some(&"2".to_string()));
}

#[test]
fn append_only_no_upsert_keeps_all_rows() {
    // Without UpsertIndex, duplicate keys should all be appended.
    let input = r#"{"id":1,"v":"a"}
{"id":2,"v":"b"}
{"id":1,"v":"c"}
{"id":1,"v":"d"}
"#;

    let (tx, rx) = sync_channel(64);
    run_jsonl_stream(Cursor::new(input.as_bytes().to_vec()), "stdin", false, tight_config(), &tx);
    drop(tx);
    let (_schema, batches, eof) = collect(rx);
    assert!(eof);

    // Apply batches with pure vstack (no upsert).
    let mut live = DataFrame::empty();
    for b in batches {
        if live.width() == 0 {
            live = b;
        } else {
            live.vstack_mut_owned(b).unwrap();
        }
    }

    // All 4 rows present — no dedup.
    assert_eq!(live.height(), 4);
    let id_col = live.column("id").unwrap();
    assert_eq!(id_col.get(0).unwrap().to_string(), "1");
    assert_eq!(id_col.get(1).unwrap().to_string(), "2");
    assert_eq!(id_col.get(2).unwrap().to_string(), "1");
    assert_eq!(id_col.get(3).unwrap().to_string(), "1");
}
