#!/bin/sh
# Which property of the item does the watcher fail to read?

HELPER=~/ahk/crates/ahk-box/helper/ahk-tray.py
IN=/tmp/tray-in

rm -f "$IN" /tmp/tray-out.txt /tmp/tray-err.txt
mkfifo "$IN"

AHK_EVENTS_FILE=/tmp/tray-events.txt python3 "$HELPER" < "$IN" \
    > /tmp/tray-out.txt 2>/tmp/tray-err.txt &
HELPER_PID=$!
exec 3>"$IN"
sleep 2

NAME="org.khronos.ahk.tray.p$HELPER_PID"
echo "name=$NAME"

for PROPERTY in Category Id Title Status IconName ItemIsMenu ToolTip; do
    : > /tmp/tray-err.txt
    gdbus call --session --dest "$NAME" --object-path /StatusNotifierItem \
        --method org.freedesktop.DBus.Properties.Get org.kde.StatusNotifierItem \
        "$PROPERTY" > /tmp/tray-prop.txt 2>&1
    echo "--- $PROPERTY"
    cat /tmp/tray-prop.txt
    if [ -s /tmp/tray-err.txt ]; then
        echo "    the log said: $(cat /tmp/tray-err.txt)"
    fi
done

echo "=== the click ==="
: > /tmp/tray-err.txt
gdbus call --session --dest "$NAME" --object-path /StatusNotifierItem \
    --method org.kde.StatusNotifierItem.Activate 0 0 2>&1
echo "--- the events file ---"
cat /tmp/tray-events.txt 2>&1
echo "--- the log ---"
cat /tmp/tray-err.txt

exec 3>&-
kill "$HELPER_PID" 2>/dev/null
rm -f "$IN"
