#!/bin/sh
# The helper of the tray, run by hand, with its error visible.
#
# The click is made from here, so the traceback of the helper is seen
# instead of a "Remote peer disconnected" from the bus.

HELPER=~/ahk/crates/ahk-box/helper/ahk-tray.py
EVENTS=/tmp/tray-events-hand.txt

rm -f "$EVENTS"
AHK_EVENTS_FILE="$EVENTS" python3 "$HELPER" > /tmp/tray-helper-out.txt 2>/tmp/tray-helper-err.txt &
HELPER_PID=$!
sleep 2

ITEM="org.khronos.ahk.tray.p$HELPER_PID"
echo "item=[$ITEM]"
echo "--- the answer of the helper ---"
cat /tmp/tray-helper-out.txt

echo "--- the click ---"
gdbus call --session --dest "$ITEM" --object-path /StatusNotifierItem \
    --method org.kde.StatusNotifierItem.Activate 0 0 2>&1 || echo "(the click was refused)"

sleep 1
echo "--- the events file ---"
cat "$EVENTS" 2>&1
echo "--- the error of the helper ---"
cat /tmp/tray-helper-err.txt
echo "--- the answer of the helper ---"
cat /tmp/tray-helper-out.txt
kill "$HELPER_PID" 2>/dev/null
