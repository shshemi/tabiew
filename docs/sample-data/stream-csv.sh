#!/usr/bin/env bash
# Producer for the --follow CSV streaming demo. Emits a header line
# followed by rows dripped at 0.5 s intervals.
#
# Columns 0 (sensor_id) and 1 (region) are STABLE identifiers suitable
# for keying. Columns 2-5 change every round.
#
# Usage examples:
#   # Key on sensor_id alone — 5 rows, updates in place
#   ./stream-csv.sh | tw -f csv --follow --key 0 --stream-batch-ms 200
#
#   # Composite key (sensor_id, region) — 10 rows (5 sensors × 2 regions)
#   ./stream-csv.sh | tw -f csv --follow --key 0,1 --stream-batch-ms 200
#
#   # Key on region alone — 2 rows (us-east, eu-west)
#   ./stream-csv.sh | tw -f csv --follow --key 1 --stream-batch-ms 200
#
# Watch the Ins/Upd counters in the tab header: Ins climbs to the
# number of unique key combinations then stops, while Upd keeps growing.

set -euo pipefail

# Header
echo "sensor_id,region,ts,temp_c,humidity,status"

ids=(sensor-1 sensor-2 sensor-3 sensor-4 sensor-5)
regions=(us-east eu-west)

for round in $(seq 1 30); do
  for id in "${ids[@]}"; do
    for region in "${regions[@]}"; do
      ts=$(date -u +%Y-%m-%dT%H:%M:%SZ)
      temp=$(awk "BEGIN{printf \"%.1f\", 18 + (${RANDOM} % 120) / 10.0}")
      humidity=$((30 + RANDOM % 50))
      status=$( ((RANDOM % 10 == 0)) && echo "warning" || echo "ok" )
      printf '%s,%s,%s,%s,%d,%s\n' "$id" "$region" "$ts" "$temp" "$humidity" "$status"
    done
  done
  sleep 0.5
done
