#!/bin/bash
# Drive the helper through a window and two controls, and look for them in the
# accessibility tree. The pipe is a fifo so the helper sees an open input and
# stays alive while the dump runs.
busctl --user set-property org.a11y.Bus /org/a11y/bus org.a11y.Status IsEnabled b true
rm -f /tmp/ahk-gui-in /tmp/ahk-gui-out
mkfifo /tmp/ahk-gui-in
python3 /tmp/ahk-gui.py < /tmp/ahk-gui-in > /tmp/ahk-gui-out 2>&1 &
HELPER=$!
exec 3> /tmp/ahk-gui-in
sleep 1
echo '{"op": "window", "title": "ahk-gui-helper-probe"}' >&3
sleep 3
echo '{"op": "add", "kind": "Edit", "text": "the field"}' >&3
echo '{"op": "add", "kind": "Button", "text": "the button"}' >&3
sleep 5
echo "=== the answers of the helper ==="
cat /tmp/ahk-gui-out
echo "=== the tree ==="
bash /tmp/atspi-dump.sh 2>&1 | grep -E "the field|the button|ahk-gui-helper" | head -6
exec 3>&-
sleep 1
kill $HELPER 2>/dev/null
busctl --user set-property org.a11y.Bus /org/a11y/bus org.a11y.Status IsEnabled b false
rm -f /tmp/ahk-gui-in
echo "=== done ==="
