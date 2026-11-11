#!/bin/bash
# Deploy the tree to the VM and probe the X11 geometry and min-max.
set -e
cd /root/ahk
cargo build --release -p ahk-cli 2>&1 | grep -cE '^error' || true
install -m 755 target/release/ahk /usr/local/bin/ahk
cat > /tmp/x6.ahk <<'AHK'
x := 0
y := 0
w := 0
h := 0
try {
  WinGetPos(&x, &y, &w, &h, "tester@archlinux")
  FileAppend "pos=[" x "," y "," w "," h "]", "*"
} catch as e1 {
  FileAppend "pos-reason=[" e1.Message "]", "*"
}
try {
  mm := WinGetMinMax("tester@archlinux")
  FileAppend "minmax=[" mm "]", "*"
} catch as e2 {
  FileAppend "minmax-reason=[" e2.Message "]", "*"
}
AHK
su - tester -c 'export DISPLAY=:0; /usr/local/bin/ahk --timeout=40 < /tmp/x6.ahk' 2>&1 | python3 -c "import json,sys; r=json.load(sys.stdin); print('X11 geo:', r.get('output'), r.get('error'))"
