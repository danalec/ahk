#!/bin/bash
# the probes of a script of the v1, under the v1 and under the tree
#
# Every probe of `v1-probes/` is run under the binary of AutoHotkey
# v1.1.37.02 and under the tree, and the answers of the two are put side
# by side. The probe that answers the same under the two is the one that
# measures the v1 right.
#
#   scripts/v1-probes.sh            # every probe
#   scripts/v1-probes.sh probe-menu # one probe
#
# The tree is built with `cargo build --release` first, and the binary of
# the v1 is at the place below (the path may be given as the first
# argument of the script).

set -u
HERE="$(cd "$(dirname "$0")" && pwd)"
PROBES="$HERE/v1-probes"
V1="${AHK_V1:-C:\Program Files\AutoHotkey\v1.1.37.02\AutoHotkeyU64.exe}"
TREE="${AHK_TREE:-$HERE/../target/release/ahk.exe}"

run() {
  local probe="$1"
  local exe="$2"
  local here
  here="$(cd "$PROBES" && pwd)"
  rm -f "$here/$probe.txt"
  if [ "${exe##*\\}" = "AutoHotkeyU64.exe" ]; then
    "$exe" "$here/$probe.ahk" > /dev/null 2>&1 &
  else
    "$exe" --dialect=v1 "$here/$probe.ahk" > /dev/null 2>&1 &
  fi
  sleep 6
  cat "$here/$probe.txt" 2>/dev/null
  taskkill //F //IM ahk.exe > /dev/null 2>&1
  taskkill //F //IM AutoHotkeyU64.exe > /dev/null 2>&1
  sleep 1
}

list=("$@")
if [ ${#list[@]} -eq 0 ]; then
  list=(probe-menu probe-timer probe-catch probe-hwnd2 probe-rcb probe-rcb2 probe-icon probe-dllstr probe-base)
fi

for probe in "${list[@]}"; do
  a="$(run "$probe" "$V1")"
  b="$(run "$probe" "$TREE")"
  if [ "$a" = "$b" ]; then
    echo "OK    $probe"
  else
    echo "DIFF  $probe"
    diff <(echo "$a") <(echo "$b") | head -14
  fi
done
