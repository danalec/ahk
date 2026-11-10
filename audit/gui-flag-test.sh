#!/bin/bash
# The flag operations of the helper: read and write Enabled and Visible.
busctl --user set-property org.a11y.Bus /org/a11y/bus org.a11y.Status IsEnabled b true
rm -f /tmp/ahk-gui-in /tmp/ahk-gui-out
mkfifo /tmp/ahk-gui-in
python3 /tmp/ahk-gui.py < /tmp/ahk-gui-in > /tmp/ahk-gui-out 2>&1 &
HELPER=$!
exec 3> /tmp/ahk-gui-in
sleep 1
echo '{"op": "window", "title": "ahk-flag-probe"}' >&3
sleep 2
echo '{"op": "add", "kind": "Edit", "text": "x"}' >&3
sleep 2
echo '{"op": "flag", "id": 1, "name": "enabled"}' >&3
sleep 1
echo '{"op": "flag", "id": 1, "name": "visible"}' >&3
sleep 1
echo '{"op": "flag", "id": 1, "name": "enabled", "on": false}' >&3
sleep 1
echo '{"op": "flag", "id": 1, "name": "visible", "on": false}' >&3
sleep 1
echo '{"op": "flag", "id": 1, "name": "enabled", "on": true}' >&3
sleep 1
echo '{"op": "flag", "id": 1, "name": "visible", "on": true}' >&3
sleep 1
echo '{"op": "flag", "id": 9, "name": "enabled"}' >&3
sleep 1
echo "=== the answers ==="
grep -E '^\{' /tmp/ahk-gui-out
exec 3>&-
sleep 1
kill $HELPER 2>/dev/null
busctl --user set-property org.a11y.Bus /org/a11y/bus org.a11y.Status IsEnabled b false
rm -f /tmp/ahk-gui-in
echo "=== done ==="
