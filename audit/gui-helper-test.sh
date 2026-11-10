#!/bin/bash
# Gui() must not show the window and Show() must, which is the difference the
# reference binary makes. The accessibility tree is the witness: a window that
# is not shown is not in it.
busctl --user set-property org.a11y.Bus /org/a11y/bus org.a11y.Status IsEnabled b true
rm -f /tmp/ahk-gui-in /tmp/ahk-gui-out
mkfifo /tmp/ahk-gui-in
python3 /tmp/ahk-gui.py < /tmp/ahk-gui-in > /tmp/ahk-gui-out 2>&1 &
HELPER=$!
exec 3> /tmp/ahk-gui-in
sleep 1
echo '{"op": "window", "title": "ahk-gui-hidden"}' >&3
sleep 3
echo -n "before show, the tree holds: "
bash /tmp/atspi-dump.sh 2>/dev/null | grep -c "ahk-gui-hidden"
echo '{"op": "add", "kind": "Edit", "text": "the field"}' >&3
echo '{"op": "show"}' >&3
sleep 4
echo -n "after show, the tree holds:  "
bash /tmp/atspi-dump.sh 2>/dev/null | grep -c "ahk-gui-hidden"
bash /tmp/atspi-dump.sh 2>/dev/null | grep -E "the field" | head -1
echo '{"op": "destroy"}' >&3
sleep 2
echo -n "after destroy, the tree holds: "
bash /tmp/atspi-dump.sh 2>/dev/null | grep -c "ahk-gui-hidden"
echo "=== the answers ==="
grep -E '^\{' /tmp/ahk-gui-out
exec 3>&-
sleep 1
kill $HELPER 2>/dev/null
busctl --user set-property org.a11y.Bus /org/a11y/bus org.a11y.Status IsEnabled b false
rm -f /tmp/ahk-gui-in
echo "=== done ==="
