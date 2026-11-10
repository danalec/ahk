#!/bin/bash
# The two new operations of the helper: read and write the Value of a control.
busctl --user set-property org.a11y.Bus /org/a11y/bus org.a11y.Status IsEnabled b true
rm -f /tmp/ahk-gui-in /tmp/ahk-gui-out
mkfifo /tmp/ahk-gui-in
python3 /tmp/ahk-gui.py < /tmp/ahk-gui-in > /tmp/ahk-gui-out 2>&1 &
HELPER=$!
exec 3> /tmp/ahk-gui-in
sleep 1
echo '{"op": "window", "title": "ahk-value-probe"}' >&3
sleep 2
echo '{"op": "add", "kind": "Edit", "text": "the first text"}' >&3
sleep 2
echo '{"op": "value", "id": 1}' >&3
sleep 2
echo '{"op": "value", "id": 1, "text": "written by the helper"}' >&3
sleep 2
echo '{"op": "value", "id": 1}' >&3
sleep 2
echo '{"op": "value", "id": 9}' >&3
sleep 2
echo "=== the answers ==="
grep -E '^\{' /tmp/ahk-gui-out
exec 3>&-
sleep 1
kill $HELPER 2>/dev/null
busctl --user set-property org.a11y.Bus /org/a11y/bus org.a11y.Status IsEnabled b false
rm -f /tmp/ahk-gui-in
echo "=== done ==="
