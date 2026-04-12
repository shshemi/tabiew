#!/usr/bin/env bash
# Producer for the --key upsert demo. Emits a small set of unique
# request_ids over and over, with the latency_ms field changing each
# time. With --key 0 (the default), Tabiew should hold a fixed-size
# table where the same rows mutate in place rather than scrolling.
#
# Usage:
#   ./stream-upserts.sh | tw -f jsonl --follow --key 0 --stream-batch-ms 200
#
# Watch the Ins/Upd counters in the tab header: Ins climbs to ~5 then
# stops, while Upd keeps growing.

set -euo pipefail

ids=(req-A req-B req-C req-D req-E)

for round in $(seq 1 30); do
  for id in "${ids[@]}"; do
    ts=$(date -u +%Y-%m-%dT%H:%M:%SZ)
    latency=$((RANDOM % 500 + 5))
    status=$((200 + (RANDOM % 3) * 100))
    printf '{"request_id":"%s","ts":"%s","latency_ms":%d,"status":%d,"round":%d}\n' \
      "$id" "$ts" "$latency" "$status" "$round"
  done
  sleep 0.5
done
