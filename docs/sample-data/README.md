# Sample data for `cli-examples.md`

Every text file referenced in `../cli-examples.md` lives here. Run the
commands directly from this directory:

```bash
cd docs/sample-data
tw data.csv
tw sales.csv customers.parquet orders.jsonl   # parquet needs generate-binary.sh
```

## What's in the box

| File | Format | Used by | Notes |
|---|---|---|---|
| `data.csv` | CSV | Sections 1, 3, 4 | 10 users — generic small dataset |
| `sales.csv` | CSV | Section 1 | 10 sales rows with date/region/amount |
| `orders.jsonl` | JSONL | Section 1 | 8 order events |
| `logs.jsonl` | JSONL | Section 1 | 10 nested API log lines |
| `file.jsonl` | JSONL | Section 1 (stdin) | Page-view events |
| `day1.csv`, `day2.csv`, `day3.csv` | CSV | Section 1 (`--multiparts`) | Same schema, 3 days of store sales |
| `mystery.txt` | CSV (no extension) | Section 2 | Demo for `-f csv` override |
| `data.log` | TSV (extension says log) | Section 2 | Demo for `-f tsv` override |
| `service.log` | logfmt | Section 2 | Demo for `-f logfmt` override |
| `data.txt` | DSV (pipe-delimited) | Section 3 | Custom separator example |
| `headerless.csv` | CSV | Section 3 | No header row |
| `messy.csv` | CSV | Section 3 | Bad cells — pair with `--ignore-errors` |
| `ragged.csv` | CSV | Section 3 | Uneven row widths — pair with `--truncate-ragged-lines` |
| `big.csv` | CSV | Section 4 | 120 rows — for `--infer-schema fast` demos |
| `mixed.csv` | CSV | Section 4 | int/float/bool/date/datetime mix |
| `events.csv` | CSV | Section 4, 7 | Has `event_date` + `occurred_at` for `--infer-datetimes` |
| `report.fwf` | FWF | Section 5 | 5 columns, widths `10,10,20,16,8` |
| `raw.txt` | DSV (pipe, no header) | Section 7 | For the combined-options example |
| `events.jsonl` | JSONL | Section 7 | Datetime-rich events |
| `app.log` | JSONL (extension says log) | Section 8 (streaming) | Tail it with `--follow` |
| `stream-jsonl.sh` | producer | Section 8 | Drips JSONL events for the streaming demo |
| `stream-upserts.sh` | producer | Section 8 | Repeats 5 JSONL keys so `--key 0` triggers updates |
| `stream-csv.sh` | producer | Section 8 | Repeats 5 CSV sensor rows so `--key 0` triggers updates |
| `generate-binary.sh` | script | Section 1, 6 | Materializes parquet/arrow/xlsx/sqlite if you have the tools |

## Binary formats

`report.parquet`, `customers.parquet`, `metrics.arrow`, `books.xlsx`, and
`app.db` are *not* committed (they are binary blobs and need optional
toolchains). Run:

```bash
./generate-binary.sh
```

The script writes whichever ones it can — parquet/arrow need
`pip install pyarrow`, xlsx needs `pip install openpyxl`, and `app.db`
needs the `sqlite3` CLI. Missing tools just skip that format.

## Running the streaming demo

The two `stream-*.sh` scripts let you exercise `--follow` and `--key`
without setting up a real log feed:

```bash
# Append-only stream (rows scroll in, no upserts)
./stream-jsonl.sh | tw -f jsonl --follow --stream-batch-ms 200

# Upsert stream (JSONL) — watch the Ins/Upd counters in the tab header
./stream-upserts.sh | tw -f jsonl --follow --key 0 --stream-batch-ms 200

# Upsert stream (CSV) — same idea, CSV format
./stream-csv.sh | tw -f csv --follow --key 0 --stream-batch-ms 200
```

You can also tail `app.log` like a real log file:

```bash
# In one terminal
tail -F app.log | tw -f jsonl --follow

# In another terminal, append more events to watch the table grow
echo '{"ts":"2024-03-15T12:00:11Z","level":"info","request_id":"req-011","method":"GET","path":"/api/users","status":200,"latency_ms":17}' >> app.log
```

## A quick smoke test

Run this after creating the directory to verify every text file parses:

```bash
for f in *.csv *.jsonl *.txt *.log *.fwf; do
  echo "== $f =="
  case "$f" in
    *.fwf)        tw "$f" -f fwf --widths 10,10,20,16,8 ;;
    headerless.*) tw "$f" --no-header ;;
    raw.*)        tw "$f" -f csv --separator '|' --no-header ;;
    data.txt)     tw "$f" -f csv --separator '|' ;;
    data.log)     tw "$f" -f tsv ;;
    service.log)  tw "$f" -f logfmt ;;
    app.log)      tw "$f" -f jsonl ;;
    messy.*)      tw "$f" --ignore-errors ;;
    ragged.*)     tw "$f" --truncate-ragged-lines ;;
    *)            tw "$f" ;;
  esac
done
```

(Hit `Q` in the TUI to advance to the next file.)
