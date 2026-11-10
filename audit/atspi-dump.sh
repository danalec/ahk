#!/bin/bash
# Dump the accessibility tree, one line per node: path, name, role and text.
# The bus address and the names are read fresh every time: they change between
# dialogs, and a probe that hard-codes one measures nothing.
A=$(busctl --user call org.a11y.Bus /org/a11y/bus org.a11y.Bus GetAddress | sed 's/^s "//; s/"$//')
echo "address=$A"

# The answer is `{"type":...,"data":"x"}` for a plain string and
# `{"type":"s","data":["x"]}` for the first value of a tuple, and both wear
# the same brackets off.
field() { sed 's/.*"data"://; s/^"//; s/"}$//; s/^\["//; s/"\]}$//'; }

walk() {
  local bus=$1 path=$2 depth=$3
  if [ "$depth" -gt 5 ]; then return; fi
  local name role text state acts ext
  name=$(busctl --address="$A" --json=short get-property "$bus" "$path" org.a11y.atspi.Accessible Name 2>/dev/null | field)
  role=$(busctl --address="$A" --json=short call "$bus" "$path" org.a11y.atspi.Accessible GetRoleName 2>/dev/null | field)
  text=$(busctl --address="$A" --json=short call "$bus" "$path" org.a11y.atspi.Text GetText ii 0 200 2>/dev/null | field)
  state=$(busctl --address="$A" --json=short call "$bus" "$path" org.a11y.atspi.Accessible GetState 2>/dev/null | sed 's/.*"data"://')
  # The names of the actions the node offers, if it offers the interface.
  acts=$(busctl --address="$A" --json=short call "$bus" "$path" org.a11y.atspi.Action GetActions 2>/dev/null | sed 's/.*"data"://')
  # The extents in all three spaces atspi offers: 0 screen, 1 window, 2 parent.
  # v2 writes client-relative coordinates into `ControlGetPos`, so which of the
  # three matches it is the question this answers.
  ext=$(busctl --address="$A" --json=short call "$bus" "$path" org.a11y.atspi.Component GetExtents u 0 2>/dev/null | sed 's/.*"data"://')
  extw=$(busctl --address="$A" --json=short call "$bus" "$path" org.a11y.atspi.Component GetExtents u 1 2>/dev/null | sed 's/.*"data"://')
  extp=$(busctl --address="$A" --json=short call "$bus" "$path" org.a11y.atspi.Component GetExtents u 2 2>/dev/null | sed 's/.*"data"://')
  printf '%*s%s name=[%s] role=[%s] text=[%s] state=%s acts=%s ext=%s win=%s par=%s\n' $((depth * 2)) '' "$path" "$name" "$role" "$text" "$state" "$acts" "$ext" "$extw" "$extp"
  local kids pair b p
  kids=$(busctl --address="$A" --json=short call "$bus" "$path" org.a11y.atspi.Accessible GetChildren 2>/dev/null)
  # The children come from a process substitution and not from a temporary
  # file: the recursion runs inside the loop, and one name for the file meant
  # the inner call deleted the file the outer loop was reading, so only the
  # first child of every node was ever walked.
  while read -r pair; do
    b=$(echo "$pair" | sed 's/\["\([^"]*\)","\([^"]*\)"\]/\1/')
    p=$(echo "$pair" | sed 's/\["\([^"]*\)","\([^"]*\)"\]/\2/')
    walk "$b" "$p" $((depth + 1))
  done < <(echo "$kids" | grep -o '\["[^"]*","[^"]*"\]')
}

walk org.a11y.atspi.Registry /org/a11y/atspi/accessible/root 0
