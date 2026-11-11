#!/bin/bash
# The event channel of the window: the helper writes what happened to a file
# the runtime polls, and the runtime calls the handler of the control.
#
# The click is written into the file the runtime reads, so this measures the
# half of the channel that is the tree's: the helper's half is the branch's
# (its round 70 proved it with a real click on the box).
set -u
RUNNER=${1:-$HOME/ahk-port/target/release/ahk}
rm -f /tmp/ahk-gui-events-*
cat > /tmp/ev.ahk <<'EOF'
out := ""
Pressed(control) {
  global out
  out .= "pressed [" control.__Class "]"
}
g := Gui()
b := g.Add("Button", "", "press me")
b.OnEvent("Click", Pressed)
g.Show()
Sleep 3000
FileAppend "[" out "]", "*"
EOF
"$RUNNER" --timeout=30 < /tmp/ev.ahk > /tmp/ev.out 2>&1 &
RUN=$!
sleep 2
PID=$(cat "$HOME/.cache/ahk/run.pid" 2>/dev/null || cat "$HOME/.local/share/ahk/run.pid" 2>/dev/null)
echo "the pid of the run is $PID"
echo 'click:1' > "/tmp/ahk-gui-events-$PID"
wait $RUN
echo "=== the report ==="
cat /tmp/ev.out
echo
