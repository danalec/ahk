#!/bin/bash
# Does a press of Escape in the window of the helper write the escape event?
busctl --user set-property org.a11y.Bus /org/a11y/bus org.a11y.Status IsEnabled b true
rm -f /tmp/ahk-gui-in /tmp/ahk-gui-out /tmp/ahk-events
mkfifo /tmp/ahk-gui-in
python3 /tmp/ahk-gui.py < /tmp/ahk-gui-in > /tmp/ahk-gui-out 2>&1 &
HELPER=$!
exec 3> /tmp/ahk-gui-in
sleep 1
echo '{"op": "window", "title": "ahk-escape-probe"}' >&3
echo '{"op": "watch", "path": "/tmp/ahk-events"}' >&3
sleep 2
echo '{"op": "add", "kind": "Edit", "text": "the field"}' >&3
echo '{"op": "show"}' >&3
sleep 4
echo "--- focusing the field, then sending Escape ---"
cat > /tmp/esc.ahk <<'AHK'
ControlFocus("the field", "ahk-escape-probe")
Sleep 300
Send("{Escape}")
Sleep 500
AHK
/home/danalec/ahk/target/release/ahk --timeout=60 < /tmp/esc.ahk 2>&1 | head -2
sleep 1
echo "=== the events ==="
cat /tmp/ahk-events 2>&1
exec 3>&-
sleep 1
kill $HELPER 2>/dev/null
busctl --user set-property org.a11y.Bus /org/a11y/bus org.a11y.Status IsEnabled b false
rm -f /tmp/ahk-gui-in
echo "=== done ==="
