"""The performance of the tree against the official v2.

    python scripts/perf-compare.py [workload ...]

Every workload of `scripts/perf/` is run under the tree and under the
official v2.0.32, and the median of the runs of each is printed in one
table.

The workload measures itself: it takes the time of the machine with
`A_TickCount` and writes it to `perf-out.txt`, in the folder of the
script. The clock of Windows counts in 15,6 milliseconds, so a workload
that is fast is measured in zeros -- the workloads are long enough for
the number to say something, and the error of a number is one tick.

The tree is the release build (`cargo build --release -p ahk-cli`) and
the official is the `ahk-v2-headless` of the reference, which is the
v2.0.32.
"""

import json
import os
import shutil
import statistics
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PERF = ROOT / "scripts" / "perf"
TREE = ROOT / "target" / "release" / "ahk.exe"
OFFICIAL = Path(os.environ.get("AHK_V2", "")) if os.environ.get("AHK_V2") else Path(
    r"C:\Users\danalec\Documents\src\ahk-v2-headless\bin\AutoHotkey64.exe"
)
RUNS = 5


def the_workloads(wanted):
    """The workloads of the run, and the ones that are not there."""
    if not wanted:
        return sorted(PERF.glob("*.ahk"))
    paths = []
    for name in wanted:
        path = PERF / (name if name.endswith(".ahk") else f"{name}.ahk")
        if not path.exists():
            raise SystemExit(f"there is no workload named {name} in {PERF}")
        paths.append(path)
    return paths


def run(engine, script, work, timeout):
    """Run a workload and answer the times that it wrote."""
    answer = work / "perf-out.txt"
    if answer.exists():
        answer.unlink()
    process = subprocess.run(
        [str(engine), str(script)],
        capture_output=True,
        text=True,
        timeout=timeout,
        cwd=str(work),
    )
    if not answer.exists():
        raise SystemExit(
            f"{engine} did not answer for {script.name}:\n{process.stdout}\n{process.stderr}"
        )
    times = []
    for line in answer.read_text(encoding="utf-8", errors="replace").splitlines():
        name, _, number = line.strip().partition(" ")
        if name and number:
            times.append((name, int(number)))
    if not times:
        raise SystemExit(f"{engine} wrote no time for {script.name}")
    return times


def measure(engine, script, runs, timeout):
    """The median of the times of a workload under an engine.

    A workload can have more than one phase: the name of the phase is
    the key, and the median is taken per phase.
    """
    phases = {}
    with tempfile.TemporaryDirectory(ignore_cleanup_errors=True) as folder:
        work = Path(folder)
        # The workload is copied: the file that it writes is in the
        # folder of the script, and the folder of the repo is not the
        # place of a measurement.
        copy = work / script.name
        shutil.copyfile(script, copy)
        for _ in range(runs):
            for name, number in run(engine, copy, work, timeout):
                phases.setdefault(name, []).append(number)
    return {name: statistics.median(numbers) for name, numbers in phases.items()}


def main():
    wanted = sys.argv[1:]
    if not TREE.exists():
        raise SystemExit(
            f"the tree is not built: {TREE} (cargo build --release -p ahk-cli)"
        )
    if not OFFICIAL.exists():
        raise SystemExit(f"the official v2 is not there: {OFFICIAL}")
    workloads = the_workloads(wanted)
    print(f"the workloads: {len(workloads)}   the runs of each: {RUNS}")
    print(f"  the tree:    {TREE}")
    print(f"  the official: {OFFICIAL}")
    print()
    print(f"{'the workload':<18}{'the tree':>12}{'the official':>16}{'the ratio':>12}")
    for script in workloads:
        ours = measure(TREE, script, RUNS, 300)
        theirs = measure(OFFICIAL, script, RUNS, 300)
        for name in ours:
            mine = ours[name]
            other = theirs.get(name, 0)
            ratio = f"{mine / other:.2f}x" if other else "-"
            print(f"{name:<18}{mine:>10} ms{other:>14} ms{ratio:>12}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
