#!/usr/bin/env bash
# Producer for the --follow CSV streaming demo. Emits a header line
# followed by rows dripped at 0.5 s intervals, with a small set of
# fixed IDs so --key 0 triggers upserts.
#
# Usage:
#   ./stream-csv.sh | tw -f csv --follow --key 0 --stream-batch-ms 200
#
# Watch the Ins/Upd counters in the tab header: Ins climbs to ~5 then
# stops, while Upd keeps growing.

set -euo pipefail

# Header
echo "sensor_id,ts,temp_c,humidity,status"

ids=(sensor-1 sensor-2 sensor-3 sensor-4 sensor-5)

for round in $(seq 1 30); do
  for id in "${ids[@]}"; do
    ts=$(date -u +%Y-%m-%dT%H:%M:%SZ)
    temp=$(awk "BEGIN{printf \"%.1f\", 18 + (${RANDOM} % 120) / 10.0}")
    humidity=$((30 + RANDOM % 50))
    status=$( ((RANDOM % 10 == 0)) && echo "warning" || echo "ok" )
    printf '%s,%s,%s,%d,%s\n' "$id" "$ts" "$temp" "$humidity" "$status"
  done
  sleep 0.5
done
