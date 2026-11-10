#!/bin/bash
# What the toolkit says about the place and the size of the window.
busctl --user set-property org.a11y.Bus /org/a11y/bus org.a11y.Status IsEnabled b true
rm -f /tmp/ahk-gui-in /tmp/ahk-gui-out
mkfifo /tmp/ahk-gui-in
python3 /tmp/ahk-gui.py < /tmp/ahk-gui-in > /tmp/ahk-gui-out 2>&1 &
HELPER=$!
exec 3> /tmp/ahk-gui-in
sleep 1
echo '{"op": "window", "title": "ahk-geometry-probe"}' >&3
sleep 2
echo '{"op": "show"}' >&3
sleep 3
echo '{"op": "geometry"}' >&3
sleep 2
echo "=== the answers ==="
grep -E '^\{' /tmp/ahk-gui-out
echo "=== what the compositor says the window is ==="
bash /tmp/atspi-dump.sh 2>/dev/null | grep -m1 "ahk-geometry-probe" | sed 's/.*ext=//'
exec 3>&-
sleep 1
kill $HELPER 2>/dev/null
busctl --user set-property org.a11y.Bus /org/a11y/bus org.a11y.Status IsEnabled b false
rm -f /tmp/ahk-gui-in
echo "=== done ==="
