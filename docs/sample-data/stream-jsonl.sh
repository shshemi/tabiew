#!/usr/bin/env bash
# A tiny producer that emits JSONL rows on a slow drip so you can demo
# Tabiew's --follow streaming mode without setting up a real log feed.
#
# Usage:
#   ./stream-jsonl.sh | tw -f jsonl --follow
#   ./stream-jsonl.sh | tw -f jsonl --follow --key 0 --stream-batch-ms 200
#
# It writes 50 events with random methods/statuses, one every 0.4s, then
# exits. Tabiew shows the stream as "closed" when this script terminates.

set -euo pipefail

methods=(GET POST PUT DELETE PATCH)
paths=(/api/users /api/orders /api/products /api/health /api/sessions /api/upload)
levels=(info info info info warn error)

for i in $(seq 1 50); do
  ts=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  method=${methods[$RANDOM % ${#methods[@]}]}
  path=${paths[$RANDOM % ${#paths[@]}]}
  level=${levels[$RANDOM % ${#levels[@]}]}
  status=$((200 + (RANDOM % 5) * 100))
  latency=$((RANDOM % 500 + 5))
  printf '{"request_id":"req-%03d","ts":"%s","level":"%s","method":"%s","path":"%s","status":%d,"latency_ms":%d}\n' \
    "$i" "$ts" "$level" "$method" "$path" "$status" "$latency"
  sleep 0.4
done
