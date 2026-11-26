#!/usr/bin/env python3
"""rfb -- the keyboard, the mouse and the screen of the guest, over RFB.

The guest of KVM is reached through the VNC of QEMU, and the protocol
of VNC (RFB) is the shortest path to the three things a lab needs: the
keys, the pointer and the framebuffer. The QEMU monitor takes the keys
and the screen too, but its mouse is relative and the guest has a
tablet, so the pointer of the monitor does not land where it is told.

    python scripts/xp/rfb.py shot out.png
    python scripts/xp/rfb.py click 500 400
    python scripts/xp/rfb.py type "hello"

The protocol is the one of RFB 3.8: the handshake, the security type
"None" (the VNC of the box has no password), the raw encoding and the
events of the pointer and of the keys.
"""

import socket
import struct
import time
import sys
import zlib

HOST = "192.168.0.10"
PORT = 5900

# The keys of X11 that the guest needs, for the characters of a command.
XK = {
    " ": 0x20, "\t": 0xff09, "\n": 0xff0d, "-": 0x2d, "=": 0x3d,
    "[": 0x5b, "]": 0x5d, ";": 0x3b, "'": 0x27, "`": 0x60,
    ",": 0x2c, ".": 0x2e, "/": 0x2f, "\\": 0x5c, "|": 0x7c,
    "?": 0x3f, ":": 0x3a, "_": 0x5f, "+": 0x2b, '"': 0x22,
    "<": 0x3c, ">": 0x3e, "!": 0x21, "@": 0x40, "#": 0x23,
    "$": 0x24, "%": 0x25, "^": 0x5e, "&": 0x26, "*": 0x2a,
    "(": 0x28, ")": 0x29,
    "ret": 0xff0d, "esc": 0xff1b, "tab": 0xff09, "spc": 0x20,
    "back": 0xff08, "del": 0xffff, "ins": 0xff63, "home": 0xff50,
    "end": 0xff57, "pgup": 0xff55, "pgdn": 0xff56,
    "up": 0xff52, "down": 0xff54, "left": 0xff51, "right": 0xff53,
    "f1": 0xffbe, "f2": 0xffbf, "f3": 0xffc0, "f4": 0xffc1,
    "f5": 0xffc2, "f6": 0xffc3, "f7": 0xffc4, "f8": 0xffc5,
    "f9": 0xffc6, "f10": 0xffc7,
    "alt": 0xffe9, "ctrl": 0xffe3, "shift": 0xffe1, "meta": 0xffeb,
}


# The keys of the layout of the United States: the key that is under a
# character, and whether the shift is with it.  The server of VNC maps a
# keysym to a key, and it drops the shift of the punctuation, so the
# shift is sent here, with the key.
SHIFTED = {
    "!": "1", "@": "2", "#": "3", "$": "4", "%": "5", "^": "6",
    "&": "7", "*": "8", "(": "9", ")": "0", "_": "-", "+": "=",
    "{": "[", "}": "]", ":": ";", '"': "'", "<": ",", ">": ".",
    "?": "/", "|": "\\", "~": "`",
}


def keysym(ch):
    """The keysym of a character of a command.

    The keysym of a printable ASCII character is the character itself
    (0x41 for "A", 0x5c for "\\"), so only the keys of the names need
    a table.
    """
    if ch in XK:
        return XK[ch]
    if 0x20 <= ord(ch) <= 0x7e:
        return ord(ch)
    raise ValueError("no keysym for %r" % ch)


def base_of(ch):
    """The key under a character, and whether the shift is with it."""
    if ch in SHIFTED:
        return SHIFTED[ch], True
    if ch.isupper():
        return ch.lower(), True
    return ch, False


class Rfb:
    def __init__(self, host=HOST, port=PORT, timeout=20):
        self.sock = socket.create_connection((host, port), timeout=timeout)
        self.sock.settimeout(timeout)
        self.width = 0
        self.height = 0
        self._handshake()

    # -- the low level ---------------------------------------------------

    def _recv(self, n):
        data = b""
        while len(data) < n:
            chunk = self.sock.recv(n - len(data))
            if not chunk:
                raise IOError("the connection is closed")
            data += chunk
        return data

    def _handshake(self):
        version = self._recv(12)
        self.sock.sendall(b"RFB 003.008\n")
        count = self._recv(1)[0]
        if count == 0:
            reason = self._recv(struct.unpack(">I", self._recv(4))[0])
            raise IOError("the server refuses: %s" % reason)
        types = self._recv(count)
        if 1 not in types:
            raise IOError("the server asks for a security the client has not")
        self.sock.sendall(b"\x01")  # None
        result = struct.unpack(">I", self._recv(4))[0]
        if result != 0:
            raise IOError("the server refuses the connection")
        self.sock.sendall(b"\x01")  # shared
        init = self._recv(24)
        (self.width, self.height) = struct.unpack(">HH", init[:4])
        # the pixel format is the one of the server: 32 bits, little endian
        # (the server sends it in the header; the client accepts it as it is)
        self._encodings = struct.pack(">BBHi", 2, 0, 1, 0)  # Raw only

    # -- the events ------------------------------------------------------

    def key(self, ch, down=True):
        sym = keysym(ch)
        self.sock.sendall(struct.pack(">BBHI", 4, 1 if down else 0, 0, sym))

    def tap(self, ch):
        self.key(ch, True)
        self.key(ch, False)

    def type(self, text, delay=0.04):
        # The shift goes with the key: the server of VNC maps a keysym
        # to a key and drops the shift of the punctuation.
        #
        # The keys go one at a time, with a pause between them: the
        # keyboard of the machine is a PS/2 one, and its buffer holds a
        # few keys only. A burst of a whole line overflows it, and the
        # machine loses the *first* keys of the line -- the line then
        # arrives without its beginning ("opy a:\..." for "copy a:\...").
        for ch in text:
            base, shifted = base_of(ch)
            if shifted:
                self.key("shift", True)
            self.key(base, True)
            self.key(base, False)
            if shifted:
                self.key("shift", False)
            if delay:
                time.sleep(delay)

    def release(self):
        """Let go of every key.

        A key that the client leaves pressed stays pressed in the guest,
        and the guest then reads every key as a shortcut: the client
        has to let go of them before it goes.
        """
        for name in ("shift", "alt", "ctrl", "meta"):
            try:
                self.key(name, False)
            except OSError:
                pass

    def pointer(self, x, y, button=0):
        mask = 1 if button == 1 else 0
        self.sock.sendall(struct.pack(">BBHH", 5, mask, x, y))

    def click(self, x, y):
        self.pointer(x, y, 0)
        self.pointer(x, y, 1)
        self.pointer(x, y, 0)

    def move(self, x, y):
        self.pointer(x, y, 0)

    # -- the framebuffer -------------------------------------------------

    def framebuffer(self):
        """The pixels of the screen, as a PNG."""
        self.sock.sendall(struct.pack(">BBHHHH", 3, 0, 0, 0, self.width, self.height))
        while True:
            message = self._recv(1)[0]
            if message != 0:
                continue
            self._recv(1)
            rectangles = struct.unpack(">H", self._recv(2))[0]
            raw = b""
            for _ in range(rectangles):
                x, y, w, h, encoding = struct.unpack(">HHHHi", self._recv(12))
                if encoding != 0:
                    raise IOError("the server sent the encoding %d" % encoding)
                raw += self._recv(w * h * 4)
            return self._png(raw, self.width, self.height)

    def _png(self, raw, width, height):
        """The pixels of the server (BGRA) as a PNG."""
        rows = bytearray()
        for line in range(height):
            rows.append(0)  # the filter "none"
            start = line * width * 4
            row = raw[start:start + width * 4]
            out = bytearray()
            for i in range(0, len(row), 4):
                out += bytes((row[i + 2], row[i + 1], row[i], 0xff))
            rows += out

        def chunk(tag, data):
            return (struct.pack(">I", len(data)) + tag + data
                    + struct.pack(">I", zlib.crc32(tag + data) & 0xffffffff))

        header = struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0)
        return (b"\x89PNG\r\n\x1a\n"
                + chunk(b"IHDR", header)
                + chunk(b"IDAT", zlib.compress(bytes(rows), 6))
                + chunk(b"IEND", b""))


def main():
    if len(sys.argv) < 2:
        raise SystemExit(__doc__)
    what = sys.argv[1]
    client = Rfb()
    try:
        if what == "shot":
            with open(sys.argv[2], "wb") as handle:
                handle.write(client.framebuffer())
            print("%s (%dx%d)" % (sys.argv[2], client.width, client.height))
        elif what == "click":
            client.click(int(sys.argv[2]), int(sys.argv[3]))
        elif what == "move":
            client.move(int(sys.argv[2]), int(sys.argv[3]))
        elif what == "type":
            client.type(sys.argv[2])
        elif what == "key":
            client.tap(sys.argv[2])
        else:
            raise SystemExit(__doc__)
    finally:
        client.release()
        client.sock.close()


if __name__ == "__main__":
    main()
