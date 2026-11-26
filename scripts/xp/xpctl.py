#!/usr/bin/env python3
"""xpctl -- drive the Windows XP machine of the lab from the host.

    python scripts/xp/xpctl.py cmd "c:\\probe.exe"
    python scripts/xp/xpctl.py shot out.png
    python scripts/xp/xpctl.py put scripts/xp/probe/target/i686-pc-windows-gnu/release/ahk-xp-probe.exe

WHY: the XP machine is a guest of KVM on the Unraid box, and it has no
network at all: the answer file of the installation says
`InstallDefaultComponents=No`, so the machine has no TCP/IP and no
adapter. The channel of the lab is therefore the floppy (the IDE one of
the guest, read by the guest as `A:`) and the QEMU monitor of the
guest, which takes the keyboard in (`sendkey`) and gives the screen out
(`screendump`).

The machine of the lab is a Windows XP SP3 with the answer file of
`unattend.flp`, an empty password for the administrator, and the
account `User`. The floppy of the guest is where a binary of the tree
travels; the screen is where the answer of it comes back.

The environment:

    XP_SSH   the ssh of the box  (default root@192.168.0.10)
    XP_KEY   the key of the box   (default ~/.ssh/rgx_unraid)
    XP_VM    the name of the guest (default WindowsXP)
"""

import json
import os
import subprocess
import sys
import tempfile

NAS = os.environ.get("XP_SSH", "root@192.168.0.10")
KEY = os.environ.get("XP_KEY", os.path.expanduser("~/.ssh/rgx_unraid"))
VM = os.environ.get("XP_VM", "WindowsXP")
FLOPPY = "/mnt/user/domains/%s/xp-floppy.img" % VM

SSH = ["ssh", "-o", "ConnectTimeout=8", "-i", KEY, "-o", "IdentitiesOnly=yes", NAS]
SCP = ["scp", "-q", "-o", "ConnectTimeout=8", "-i", KEY, "-o", "IdentitiesOnly=yes"]

# The key names of the QEMU monitor, for the characters of a command.
PLAIN = {
    " ": "spc", "\t": "tab", "\n": "ret", "-": "minus", "=": "equal",
    "[": "bracket_left", "]": "bracket_right", ";": "shift-semicolon",
    "'": "apostrophe", "`": "grave_accent", ",": "comma", ".": "dot",
    "/": "slash", "\\": "backslash", "|": "shift-backslash",
    "?": "shift-slash", ":": "shift-semicolon", "_": "shift-minus",
    "+": "shift-equal", '"': "shift-apostrophe", "<": "shift-comma",
    ">": "shift-dot", "!": "shift-1", "@": "shift-2", "#": "shift-3",
    "$": "shift-4", "%": "shift-5", "^": "shift-6", "&": "shift-7",
    "*": "shift-8", "(": "shift-9", ")": "shift-0",
}


def keys_for(text):
    """The key names of a string, one call each."""
    out = []
    for ch in text:
        if ch.isalpha():
            out.append(ch if ch.islower() else "shift-" + ch.lower())
        elif ch.isdigit():
            out.append(ch)
        elif ch in PLAIN:
            out.append(PLAIN[ch])
        else:
            raise SystemExit("no key name for %r" % ch)
    return out


def monitor(command):
    """One command of the QEMU monitor of the guest."""
    return subprocess.run(
        SSH + ["virsh qemu-monitor-command %s --hmp %r" % (VM, command)],
        check=True, capture_output=True, text=True,
    )


def send(keys):
    """Send the keys through the monitor, one ssh session for all of them.

    The guest drops the keys that arrive too fast, so every key waits a
    little before the next one.
    """
    script = "; ".join(
        "virsh qemu-monitor-command %s --hmp 'sendkey %s' >/dev/null; sleep 0.08"
        % (VM, k)
        for k in keys
    )
    subprocess.run(SSH + [script], check=True, capture_output=True)


def click(x, y):
    """A click of the mouse of the guest, in the pixels of its screen.

    The monitor of QEMU has no absolute mouse: `mouse_move` is a
    relative move, and `mouse_set` chooses the device, not a place.
    So the pointer is pushed to the corner first (the move is clamped
    by the machine), and then it is walked to the place of the click.
    """
    subprocess.run(
        SSH + ["virsh qemu-monitor-command %s --hmp 'mouse_move -4000 -4000; "
               "mouse_move %d %d; mouse_button 1' >/dev/null" % (VM, x, y)],
        check=True, capture_output=True,
    )


def shot(path):
    """A screenshot of the guest, through the monitor and over ssh."""
    remote = "/tmp/xp-shot.ppm"
    monitor("screendump %s" % remote)
    local = os.path.join(tempfile.gettempdir(), "xp-shot.ppm")
    subprocess.run(SCP + [NAS + ":" + remote, local], check=True, capture_output=True)
    subprocess.run(["magick", local, path], check=True)
    print(path)


def put(path):
    """Put a file on the floppy of the guest, and attach the floppy."""
    name = os.path.basename(path)
    subprocess.run(SCP + [path, NAS + ":/tmp/" + name], check=True, capture_output=True)
    script = (
        "rm -f {img}; "
        "dd if=/dev/zero of={img} bs=512 count=2880 2>/dev/null; "
        "mkfs.vfat -F 12 {img} >/dev/null; "
        "mcopy -i {img} /tmp/{name} ::; "
        "mdir -i {img}"
    ).format(img=FLOPPY, name=name)
    out = subprocess.run(SSH + [script], check=True, capture_output=True, text=True)
    print(out.stdout.strip())
    subprocess.run(
        SSH + ["virsh change-media %s fda %s --live" % (VM, FLOPPY)],
        check=True, capture_output=True,
    )
    print("the floppy of the guest carries %s" % name)


def main():
    if len(sys.argv) < 2:
        raise SystemExit(__doc__)
    what = sys.argv[1]
    if what == "cmd":
        # A command line in the console that is open, and the return.
        send(keys_for(sys.argv[2]) + ["ret"])
    elif what == "click":
        # A click of the mouse of the guest, in the pixels of its screen.
        click(int(sys.argv[2]), int(sys.argv[3]))
    elif what == "fill":
        # A field of the guest: the click in it, then the text.
        click(int(sys.argv[2]), int(sys.argv[3]))
        send(keys_for(sys.argv[4]))
    elif what == "keys":
        # The named keys, one call each: `xpctl.py keys ret ctrl-c`.
        send(sys.argv[2:])
    elif what == "console":
        # A console window of its own, and a command line in it.
        import time
        send(["meta_l", "r"])
        time.sleep(1.0)
        send(keys_for(sys.argv[2]) + ["ret"])
        time.sleep(1.0)
        send(keys_for(sys.argv[3]) + ["ret"])
    elif what == "shot":
        shot(sys.argv[2])
    elif what == "put":
        put(sys.argv[2])
    else:
        raise SystemExit(__doc__)


if __name__ == "__main__":
    main()
