#!/bin/bash
# Drive the whole protocol of the helper: a window, a control, the title read
# and written, and the window taken away.
busctl --user set-property org.a11y.Bus /org/a11y/bus org.a11y.Status IsEnabled b true
rm -f /tmp/ahk-gui-in /tmp/ahk-gui-out
mkfifo /tmp/ahk-gui-in
python3 /tmp/ahk-gui.py < /tmp/ahk-gui-in > /tmp/ahk-gui-out 2>&1 &
HELPER=$!
exec 3> /tmp/ahk-gui-in
sleep 1
echo '{"op": "window", "title": "ahk-gui-protocol"}' >&3
sleep 2
echo '{"op": "add", "kind": "Edit", "text": "field"}' >&3
echo '{"op": "title"}' >&3
sleep 2
echo '{"op": "title", "set": "ahk-gui-renamed"}' >&3
sleep 2
echo "=== the tree before the destroy ==="
bash /tmp/atspi-dump.sh 2>&1 | grep -E "ahk-gui-renamed" | head -2
echo '{"op": "destroy"}' >&3
sleep 2
echo "=== the tree after the destroy ==="
bash /tmp/atspi-dump.sh 2>&1 | grep -cE "ahk-gui-renamed"
echo "=== the answers ==="
cat /tmp/ahk-gui-out
exec 3>&-
sleep 1
kill $HELPER 2>/dev/null
busctl --user set-property org.a11y.Bus /org/a11y/bus org.a11y.Status IsEnabled b false
rm -f /tmp/ahk-gui-in
echo "=== done ==="
