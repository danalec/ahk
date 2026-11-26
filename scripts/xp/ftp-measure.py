#!/usr/bin/env python3
"""Generate the guest FTP pull/push scripts for the XP measurement.

The guest's ftp.exe does not enumerate with mget/mput against the NAS
pyftpdlib server, but single get/put work. So this writes one get per
corpus file (pull) and one put per result stream (push).

    python scripts/xp/ftp-measure.py <corpus-dir> <out-dir>

<corpus-dir> holds v1/*.ahk and v2/*.ahk (and run.bat). The scripts are
written CRLF so the XP ftp.exe reads them. See scripts/xp/ftp.md.
"""
import pathlib
import sys

HOST = "192.168.250.1"


def main() -> int:
    if len(sys.argv) < 3:
        print(__doc__)
        return 2
    corpus = pathlib.Path(sys.argv[1])
    out = pathlib.Path(sys.argv[2])
    out.mkdir(parents=True, exist_ok=True)

    pull = ["open " + HOST, "anonymous", "guest@x", "binary", "prompt"]
    for sub in ("v1", "v2"):
        pull += ["lcd c:\\nb\\" + sub, "cd nb/" + sub]
        pull += ["get " + f.name for f in sorted((corpus / sub).glob("*.ahk"))]
    pull += ["lcd c:\\nb", "cd ..", "get run.bat", "bye"]
    (out / "pull.txt").write_bytes(("\r\n".join(pull) + "\r\n").encode())

    push = ["open " + HOST, "anonymous", "guest@x", "binary",
            "lcd c:\\res", "cd all",
            "put v1t.txt", "put v2t.txt", "put v1r.txt", "put v1r.err",
            "put v1code.txt", "bye"]
    (out / "push.txt").write_bytes(("\r\n".join(push) + "\r\n").encode())
    print("pull lines:", len(pull), "push lines:", len(push))
    return 0


if __name__ == "__main__":
    sys.exit(main())
