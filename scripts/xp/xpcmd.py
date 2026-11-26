#!/usr/bin/env python3
"""xpcmd -- a command line in the console of the XP machine.

    python scripts/xp/xpcmd.py "mkdir c:\drv" "dir c:\drv"

Every argument is a line typed in the console of the guest, one after
the other, and the screen of the guest is left for the reading.
"""

import os
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rfb  # noqa: E402


def main():
    client = rfb.Rfb()
    for line in sys.argv[1:]:
        client.type(line)
        client.tap("\n")
        time.sleep(1.5)


if __name__ == "__main__":
    main()
