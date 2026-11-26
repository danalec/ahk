# The FTP channel for the XP lab

The guest is air-gapped by the unattend, but once its network stack is
repaired it has a routable address on the `xp-isolated` libvirt network
(`192.168.250.0/24`, NAS bridge `192.168.250.1`). FTP then moves proper
files and replaces the fragile floppy for bulk transfer.

## Repair the guest networking (one time)

The unattend (`InstallDefaultComponents=No`) strips networking. The NIC
must be a model XP drives in-box: use `pcnet` (not `e1000`, which has no
in-box XP driver). Install TCP/IP and assign a static address with the
fully-qualified `netsh` form (the positional form is rejected on XP):

```
netsh interface ip set address name="Local Area Connection 2" ^
  source=static addr=192.168.250.2 mask=255.255.255.0 gateway=192.168.250.1 gwmetric=1
```

Boot from the hard disk (detach the bootable install/driver CDs), else
the guest stalls at "Press any key to boot from CD" and never runs the
automation. Dismiss the Found New Hardware wizard so it cannot block
the Run dialog.

## The server (NAS) and the client (guest)

Run the server on the NAS with `pyftpdlib` (anonymous, write enabled):

```
pip3 install pyftpdlib
python3 -m pyftpdlib -p 21 -w -d /tmp/ftproot -i 0.0.0.0
```

The guest uses its built-in `ftp.exe` with a script (`ftp -s:pull.txt`).
**`mget`/`mput` do not enumerate against this server; single `get`/`put`
work.** So generate one `get`/`put` line per file (see
`ftp-measure.py`), not a wildcard.

`pull.txt` (guest pulls the corpus):

```
open 192.168.250.1
anonymous
guest@x
binary
prompt
lcd c:\nb\v1
cd nb/v1
get c0000.ahk
... one get per case ...
bye
```

`push.txt` (guest pushes the packed results):

```
open 192.168.250.1
anonymous
guest@x
binary
lcd c:\res
cd all
put v1t.txt
put v2t.txt
bye
```

The host then reads `/tmp/ftproot/all/*.txt` and compares against the
host tree (`scripts/xp/driver.bat` produces the streams on the guest).
