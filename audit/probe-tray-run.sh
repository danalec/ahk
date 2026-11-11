#!/bin/sh
# The tray of the script, measured on the box.
#
# The script puts its icon in the tray of the session and waits in small
# slices. The click on the icon is made from here, through the bus of the
# item, because the compositor is what answers a physical click and this
# is the link that is ours to prove.

set -e
RUNNER=~/ahk/target/release/ahk
OUT=/tmp/tray-out.json

rm -f "$OUT"
(cat /tmp/probe-tray.ahk | "$RUNNER" --timeout=60 > "$OUT" 2>&1 &)

sleep 3

echo "--- the bus names of the session ---"
gdbus call --session --dest org.freedesktop.DBus --object-path /org/freedesktop/DBus \
    --method org.freedesktop.DBus.ListNames | tr ',' '\n' | grep -i khronos || echo "(no name of ours)"

ITEM=$(gdbus call --session --dest org.freedesktop.DBus --object-path /org/freedesktop/DBus \
    --method org.freedesktop.DBus.ListNames | tr ',' '\n' | grep -i khronos | tr -d " '\"" || true)
# The item of THIS run is the newest one: the name carries the number of
# the helper, and the helpers of the runs that came before are still in
# the session for a while.
ITEM=$(echo "$ITEM" | sed 's/.*p//' | sort -n | tail -1 | sed 's/^/org.khronos.ahk.tray.p/')
echo "item=[$ITEM]"

if [ -n "$ITEM" ]; then
    echo "--- the item is in the watcher ---"
    gdbus call --session --dest org.kde.StatusNotifierWatcher --object-path /StatusNotifierWatcher \
        --method org.freedesktop.DBus.Properties.Get org.kde.StatusNotifierWatcher \
        RegisteredStatusNotifierItems 2>&1 | tr ',' '\n' | grep -i khronos || echo "(not registered)"

    echo "--- the click ---"
    gdbus call --session --dest "$ITEM" --object-path /StatusNotifierItem \
        --method org.kde.StatusNotifierItem.Activate 0 0 2>&1 || echo "(the click was refused)"
fi

sleep 5
echo "--- the report ---"
cat "$OUT"
