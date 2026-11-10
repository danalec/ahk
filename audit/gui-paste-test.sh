#!/bin/bash
# Which half hangs? The helper's paste operation, answered on its own through the
# fifo, with a timeout around the whole thing.
rm -f /tmp/ahk-gui-in /tmp/ahk-gui-out
mkfifo /tmp/ahk-gui-in
python3 /tmp/ahk-gui.py < /tmp/ahk-gui-in > /tmp/ahk-gui-out 2>&1 &
HELPER=$!
exec 3> /tmp/ahk-gui-in
sleep 1
echo '{"op": "window", "title": "paste-probe"}' >&3
sleep 2
echo '{"op": "add", "kind": "Edit", "text": "start"}' >&3
sleep 2
echo '{"op": "show"}' >&3
sleep 2
echo "--- sending the paste ---"
echo '{"op": "paste", "id": 1, "text": " MORE"}' >&3
echo "--- waiting up to 8 seconds for an answer ---"
for i in $(seq 1 8); do
  sleep 1
  if grep -q '"value": "start MORE"' /tmp/ahk-gui-out 2>/dev/null; then
    echo "ANSWERED after ${i}s"
    grep -E '^\{' /tmp/ahk-gui-out
    break
  fi
done
echo "--- the answers so far ---"
grep -E '^\{' /tmp/ahk-gui-out
exec 3>&-
sleep 1
kill $HELPER 2>/dev/null
rm -f /tmp/ahk-gui-in
echo "=== done ==="
