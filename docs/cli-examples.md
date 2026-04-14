# Tabiew CLI Reference & Examples

Every command below invokes the `tw` binary. This document walks through
every command-line flag Tabiew accepts, grouped by purpose, with runnable
examples. Streaming features added in the `feat/streaming-stdin` branch
(`--follow`, `--key`, `--stream-batch-rows`, `--stream-batch-ms`) are
called out in their own section at the end.

> **Sample data:** every filename in this document maps to a real file
> under [`./sample-data/`](./sample-data/). `cd docs/sample-data` and the
> commands below run as-is. Binary formats (`.parquet`, `.arrow`,
> `.xlsx`, `.db`) are produced by `./generate-binary.sh`; see
> [`sample-data/README.md`](./sample-data/README.md).

---

## Synopsis

```
tw [FILES]... [OPTIONS]
```

Tabiew opens one or more tabular files in a TUI. With no positional
arguments it reads from stdin; with `--follow` it reads *progressively*
from stdin so rows appear as they arrive instead of after EOF.

---

## 1. Opening files

### Open a single file (format auto-detected by extension)
```bash
tw data.csv
tw report.parquet
tw logs.jsonl
tw metrics.arrow
tw books.xlsx
tw app.db
```

### Open multiple files (each becomes its own tab)
```bash
tw sales.csv customers.parquet orders.jsonl
```

### Concatenate multiple files vertically into one tab
Use `--multiparts` when the inputs share a schema and should be stacked.
```bash
tw --multiparts day1.csv day2.csv day3.csv
```

### Read from stdin (eager — waits for EOF)
```bash
curl -s https://example.com/data.csv | tw -f csv
cat file.jsonl | tw -f jsonl
```

---

## 2. Format override (`-f` / `--format`)

Force a specific parser when the extension is wrong, missing, or the
input is piped. Valid values: `dsv`, `csv`, `tsv`, `parquet`, `jsonl`,
`json`, `arrow`, `fwf`, `sqlite`, `excel`, `logfmt`.

```bash
# File has no extension
tw mystery.txt -f csv

# Force TSV regardless of extension
tw data.log -f tsv

# Parse a .log as logfmt
tw service.log -f logfmt
```

---

## 3. Delimited-text options (CSV / TSV / DSV)

### Custom separator
```bash
tw data.txt -f csv --separator '|'
tw data.txt -f dsv --separator ';'
```

### Custom quote character
```bash
tw data.csv --quote-char "'"
```

### Input has no header row
```bash
tw headerless.csv --no-header
```

### Tolerate parse errors / ragged lines
```bash
tw messy.csv --ignore-errors
tw ragged.csv --truncate-ragged-lines
```

---

## 4. Schema & type inference

### Pick an inference strategy (`--infer-schema`)
- `safe` (default) — scan the whole file, never guess wrong
- `fast` — sample the first 128 rows
- `no` — treat every column as a string

```bash
tw big.csv --infer-schema fast
tw mixed.csv --infer-schema no
```

### Choose which types to try when inferring (`--infer-types`)
Space-separated list of `int`, `float`, `boolean`, `date`, `datetime`,
or `all`.
```bash
# Only try int/float (default)
tw data.csv --infer-types "int float"

# Also parse booleans and dates
tw data.csv --infer-types "int float boolean date"

# Try everything
tw data.csv --infer-types all
```

### Parse date & datetime columns (`--infer-datetimes`)
```bash
tw events.csv --infer-datetimes
```

### Disable type inference entirely (`--no-type-inference`)
```bash
tw data.csv --no-type-inference
```

---

## 5. Fixed-width files (FWF)

FWF needs you to tell Tabiew either the exact column widths or let it
infer them from whitespace alignment.

### Explicit widths
```bash
tw report.fwf -f fwf --widths 10,10,20,16,8
```

### Custom separator length between fields (default 1)
```bash
tw report.fwf -f fwf --widths 10,10,20,16,8 --separator-length 0
```

### Require strict width enforcement (no flexibility on short rows)
```bash
tw report.fwf -f fwf --widths 10,10,20,16,8 --no-flexible-width
```

### FWF file with no header row
```bash
tw report.fwf -f fwf --widths 10,10,20,16,8 --no-header
```

---

## 6. SQLite

### Open an encrypted SQLite database
```bash
tw secure.db --sqlite-key "mypassword"
```

---

## 7. Combining options

### CSV from stdin, pipe-delimited, no header, tolerate errors
```bash
cat raw.txt | tw -f csv --separator '|' --no-header --ignore-errors
```

### JSONL with fast inference, datetime parsing
```bash
tw events.jsonl --infer-schema fast --infer-datetimes
```

---

## 8. Live streaming from stdin (new in `feat/streaming-stdin`)

The streaming pipeline lets Tabiew *tail* a line-oriented feed and
render rows as they arrive, without waiting for EOF. It works for
`csv`, `tsv`, `dsv`, `jsonl`, `logfmt`, and `fwf`.

### `--follow` / `-F` — enable progressive streaming

```bash
# Tail a JSON-lines log
tail -F app.log | tw -f jsonl --follow

# Tail a CSV pipe
producer | tw -f csv --follow

# Tail a logfmt stream from kubectl
kubectl logs -f my-pod | tw -f logfmt --follow

# Tail a fixed-width feed (widths are required for streaming FWF)
producer | tw -f fwf --follow --widths 10,5,20
```

Guard rails (clap will reject invalid combinations at startup):
- `--follow` requires data piped on stdin (running it in a terminal is
  an error — Tabiew refuses to wait on `/dev/tty`).
- `--follow` is rejected for non-line-oriented formats
  (`parquet`, `arrow`, `sqlite`, `excel`, `json`).
- Streaming FWF requires `--widths` — width inference needs the whole
  file up front.

### `--no-key` — append-only streaming (no upserts)

By default `--follow` upserts on column 0. Pass `--no-key` to disable
upsert logic entirely — every incoming row is appended, even if it
shares a key with an existing row. Useful for log tailing or
append-only event streams.

```bash
# Append every row, no deduplication
tail -F access.log | tw -f csv --follow --no-key

# JSONL event stream — keep all events
producer | tw -f jsonl --follow --no-key
```

`--no-key` is mutually exclusive with `--key` and requires `--follow`.

### `--key` — composite primary key for in-place upserts

With `--key`, rows are keyed on one or more columns; when a later row
repeats an existing key Tabiew **updates the matching row in place**
(last-write-wins) instead of appending. New keys still append as usual.

`--key` takes comma-separated 0-based column indexes and defaults to
`0` (first column). It requires `--follow` and is mutually exclusive
with `--no-key`.

```bash
# Default: key on column 0
events | tw -f jsonl --follow --key 0

# Composite key on columns 1 and 3
stream-metrics | tw -f jsonl --follow --key 1,3

# Logfmt stream keyed on the first field
kubectl logs -f my-pod | tw -f logfmt --follow --key 0

# CSV feed keyed on (region, host)
producer | tw -f csv --follow --key 2,4
```

Semantics notes:
- Indexes are **0-based** and refer to the position of the column in
  the file, not its name.
- The dtypes of key columns are **locked** on the first batch; a later
  batch that sends a different dtype for any key column is a stream
  error.
- Within a single batch, last-write-wins: if the batch contains two
  rows with the same key, only the last one is inserted/updated.
- There is no `--no-key` escape hatch — always at least one key column.
- There is no delete/tombstone semantics — only inserts and updates.

### `--stream-batch-rows` — flush threshold by row count (default 1000)

The streaming reader buffers rows and flushes a batch to the UI when
either the row threshold *or* the time threshold is hit, whichever
comes first.

```bash
# Flush every 50 rows for a snappier UI on a slow feed
slow-producer | tw -f jsonl --follow --stream-batch-rows 50

# Large batches for a firehose
firehose | tw -f csv --follow --stream-batch-rows 10000
```

### `--stream-batch-ms` — flush threshold by wall time (default 250ms)

```bash
# Flush at most every second
producer | tw -f jsonl --follow --stream-batch-ms 1000

# Very responsive UI on a trickle feed
producer | tw -f jsonl --follow --stream-batch-ms 100 --stream-batch-rows 10
```

### Full-featured streaming examples

```bash
# Tail Kubernetes logs, parse as logfmt, dedupe by request_id
kubectl logs -f deploy/api | \
  tw -f logfmt --follow --key 0 --stream-batch-ms 500

# Watch a metrics feed keyed on (host, metric) with tight batches
./emit-metrics | \
  tw -f jsonl --follow --key 0,1 \
     --stream-batch-rows 100 --stream-batch-ms 200

# Follow a fixed-width accounting stream with composite key (acct,date)
./gl-stream | \
  tw -f fwf --follow --widths 12,10,8,15 --key 0,1
```

### `--flash-ms` — cell flash duration (default 750ms)

When upsert mode is active (`--key`), changed cells briefly flash:
green for inserts, yellow for updates. Adjust the duration with
`--flash-ms`:

```bash
# Shorter flash for fast-moving data
firehose | tw -f jsonl --follow --key 0 --flash-ms 300

# Longer flash to spot changes
slow-feed | tw -f csv --follow --key 0 --flash-ms 2000
```

### `--flash-color` — custom update highlight color (default yellow)

Accepts named colors (`red`, `green`, `yellow`, `blue`, `magenta`,
`cyan`, `white`, `black`) or hex (`#FF8800`):

```bash
# Orange flash for updates
producer | tw -f jsonl --follow --key 0 --flash-color '#FF8800'

# Cyan flash
producer | tw -f csv --follow --key 0 --flash-color cyan
```

### `--no-flash` — disable cell flash highlighting

```bash
# No flash, just silent upserts
producer | tw -f jsonl --follow --key 0 --no-flash
```

`--no-flash`, `--flash-ms`, and `--flash-color` are mutually exclusive
with `--no-flash`.

---

## 9. Built-in help

```bash
# Print the full CLI help from clap
tw --help

# Print version
tw --version
```

---

## 10. Once inside the TUI

CLI flags get you as far as loading the data. Inside the TUI you can:

| Key                       | Action                           |
|---------------------------|----------------------------------|
| `F1`                      | Show full keybinding help        |
| `:` or `Q <sql>`          | Open command palette / run SQL   |
| `/`                       | Fuzzy search                     |
| `h j k l` / arrows        | Navigate                         |
| `g` / `G`                 | First / last row                 |
| `Ctrl+u` / `Ctrl+d`       | Half page up / down              |
| `q`                       | Close sheet or quit              |
| `Q` (uppercase)           | Quit application                 |

Common commands at the `:` palette:
```
Q SELECT * FROM df WHERE price > 1000
S name, price, quantity
F bedrooms > 2 AND price < 500000
O -price
tabn SELECT region, SUM(amount) FROM df GROUP BY region
schema
reset
help
```

While a stream is live, the tab header shows indicator tags:
`Stream` (live/closed), `StreamRows` (total rows received),
`Ins` / `Upd` (upsert counters), and `Pending` when rows are buffered
above a frozen derived view.
