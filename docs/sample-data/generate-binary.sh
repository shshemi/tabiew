#!/usr/bin/env bash
# Generates the binary sample files referenced in cli-examples.md:
#   - report.parquet  (from sales.csv)
#   - customers.parquet
#   - metrics.arrow   (from logs.jsonl)
#   - books.xlsx
#   - app.db          (sqlite, with `users` and `orders` tables)
#
# These are NOT committed to git because they are binary blobs and
# require optional toolchains. Run this script once to materialize them.
#
# Requirements (any subset will do; the script skips formats whose tools
# are missing):
#   - python3 with pyarrow      → parquet, arrow
#   - python3 with openpyxl     → xlsx
#   - sqlite3                   → sqlite db

set -euo pipefail
cd "$(dirname "$0")"

have() { command -v "$1" >/dev/null 2>&1; }
py_has() { python3 -c "import $1" 2>/dev/null; }

# ---------- parquet ----------
if py_has pyarrow; then
  echo "[+] writing report.parquet from sales.csv"
  python3 - <<'PY'
import csv, pyarrow as pa, pyarrow.parquet as pq
rows = list(csv.DictReader(open("sales.csv")))
table = pa.table({
    "order_id": [int(r["order_id"]) for r in rows],
    "date":     [r["date"]            for r in rows],
    "product":  [r["product"]         for r in rows],
    "region":   [r["region"]          for r in rows],
    "amount":   [float(r["amount"])   for r in rows],
    "quantity": [int(r["quantity"])   for r in rows],
})
pq.write_table(table, "report.parquet")
PY

  echo "[+] writing customers.parquet"
  python3 - <<'PY'
import pyarrow as pa, pyarrow.parquet as pq
table = pa.table({
    "customer_id": list(range(1, 11)),
    "name":  ["Alice","Bob","Charlie","Diana","Eve","Frank","Grace","Henry","Iris","Jack"],
    "email": [f"user{i}@example.com" for i in range(1, 11)],
    "tier":  ["gold","silver","gold","bronze","silver","bronze","gold","silver","bronze","gold"],
    "lifetime_value": [1250.50, 890.25, 3400.00, 215.75, 1100.00, 320.50, 4200.00, 750.00, 180.25, 2800.75],
})
pq.write_table(table, "customers.parquet")
PY
else
  echo "[-] skipping parquet (install: pip install pyarrow)"
fi

# ---------- arrow ----------
if py_has pyarrow; then
  echo "[+] writing metrics.arrow"
  python3 - <<'PY'
import pyarrow as pa
table = pa.table({
    "ts":      ["2024-03-15T08:00:01Z","2024-03-15T08:00:02Z","2024-03-15T08:00:03Z","2024-03-15T08:00:04Z","2024-03-15T08:00:05Z"],
    "host":    ["web-01","web-01","web-02","web-02","web-03"],
    "metric":  ["cpu","memory","cpu","memory","cpu"],
    "value":   [0.42, 0.68, 0.31, 0.75, 0.55],
})
with pa.OSFile("metrics.arrow", "wb") as sink:
    with pa.ipc.new_file(sink, table.schema) as writer:
        writer.write_table(table)
PY
else
  echo "[-] skipping arrow (install: pip install pyarrow)"
fi

# ---------- xlsx ----------
if py_has openpyxl; then
  echo "[+] writing books.xlsx"
  python3 - <<'PY'
from openpyxl import Workbook
wb = Workbook()
ws = wb.active
ws.title = "books"
ws.append(["isbn","title","author","year","price"])
rows = [
    ("978-0-13-468599-1","The Rust Programming Language","Klabnik & Nichols",2019,39.95),
    ("978-1-59327-828-1","Programming Rust","Blandy & Orendorff",2021,59.99),
    ("978-0-321-56384-2","The C++ Programming Language","Stroustrup",2013,74.99),
    ("978-0-13-110362-7","The C Programming Language","K & R",1988,49.99),
    ("978-0-596-51748-6","JavaScript: The Good Parts","Crockford",2008,29.99),
]
for r in rows:
    ws.append(r)
wb.save("books.xlsx")
PY
else
  echo "[-] skipping xlsx (install: pip install openpyxl)"
fi

# ---------- sqlite ----------
if have sqlite3; then
  echo "[+] writing app.db"
  rm -f app.db
  sqlite3 app.db <<'SQL'
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    created_at TEXT NOT NULL
);
INSERT INTO users VALUES
    (1,'Alice','alice@example.com','2023-01-15'),
    (2,'Bob','bob@example.com','2023-02-20'),
    (3,'Charlie','charlie@example.com','2023-03-10'),
    (4,'Diana','diana@example.com','2023-04-05'),
    (5,'Eve','eve@example.com','2023-05-12');

CREATE TABLE orders (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    product TEXT NOT NULL,
    amount REAL NOT NULL,
    placed_at TEXT NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id)
);
INSERT INTO orders VALUES
    (101,1,'Widget',129.50,'2024-01-10'),
    (102,2,'Gadget',24.99,'2024-01-11'),
    (103,1,'Sprocket',312.45,'2024-01-12'),
    (104,3,'Gizmo',89.00,'2024-01-13'),
    (105,4,'Widget',215.75,'2024-01-14');
SQL
else
  echo "[-] skipping sqlite (install your platform's sqlite3 CLI)"
fi

echo "done. files in $(pwd):"
ls -la *.parquet *.arrow *.xlsx *.db 2>/dev/null || true
