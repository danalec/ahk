#!/bin/bash
# The first half of the event channel: the helper writes what happened to a file
# the runtime can poll. The click comes from the tree's own ControlClick, so the
# loop is the real one and not a signal fired by hand.
busctl --user set-property org.a11y.Bus /org/a11y/bus org.a11y.Status IsEnabled b true
rm -f /tmp/ahk-gui-in /tmp/ahk-gui-out /tmp/ahk-events
mkfifo /tmp/ahk-gui-in
python3 /tmp/ahk-gui.py < /tmp/ahk-gui-in > /tmp/ahk-gui-out 2>&1 &
HELPER=$!
exec 3> /tmp/ahk-gui-in
sleep 1
echo '{"op": "window", "title": "ahk-event-probe"}' >&3
echo '{"op": "watch", "path": "/tmp/ahk-events"}' >&3
sleep 2
echo '{"op": "add", "kind": "Button", "text": "press me"}' >&3
echo '{"op": "show"}' >&3
sleep 4
echo '--- clicking the button through the accessibility layer ---'
printf 'ControlClick("press me", "ahk-event-probe")\nSleep 500\n' > /tmp/click.ahk
/home/danalec/ahk/target/release/ahk --timeout=60 < /tmp/click.ahk > /dev/null 2>&1
sleep 1
echo "=== the events so far ==="
cat /tmp/ahk-events 2>&1
echo '{"op": "destroy"}' >&3
sleep 2
echo "=== after the destroy ==="
cat /tmp/ahk-events 2>&1
exec 3>&-
sleep 1
kill $HELPER 2>/dev/null
busctl --user set-property org.a11y.Bus /org/a11y/bus org.a11y.Status IsEnabled b false
rm -f /tmp/ahk-gui-in
echo "=== done ==="
