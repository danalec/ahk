#!/bin/sh
# The helper of the tray, by hand, with its error visible.
#
# The commands come through a FIFO, so the standard input of the helper is
# a pipe that stays open -- and the helper is not blocked in a read.

HELPER=~/ahk/crates/ahk-box/helper/ahk-tray.py
IN=/tmp/tray-in

rm -f "$IN" /tmp/tray-out.txt /tmp/tray-err.txt
mkfifo "$IN"

AHK_EVENTS_FILE=/tmp/tray-events.txt python3 "$HELPER" < "$IN" \
    > /tmp/tray-out.txt 2>/tmp/tray-err.txt &
HELPER_PID=$!

# The write end is held open for the whole run.
exec 3>"$IN"

sleep 2
echo "--- the answer of the start ---"
cat /tmp/tray-out.txt
echo "--- the name of the item is owned? ---"
gdbus call --session --dest org.freedesktop.DBus --object-path /org/freedesktop/DBus \
    --method org.freedesktop.DBus.ListNames | tr ',' '\n' | grep khronos || echo "(no name)"

echo "--- the icon ---"
echo '{"op":"icon","name":"application-x-executable"}' >&3
sleep 2
cat /tmp/tray-out.txt

echo "--- the error of the helper ---"
cat /tmp/tray-err.txt
echo "--- the helper is alive? ---"
kill -0 "$HELPER_PID" 2>/dev/null && echo yes || echo "no"

exec 3>&-
kill "$HELPER_PID" 2>/dev/null
rm -f "$IN"
