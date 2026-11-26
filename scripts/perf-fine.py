"""The fine clock of the performance: the elapsed time of the tree.

    python scripts/perf-fine.py [workload ...]

The clock of the machine counts in 15,6 milliseconds, and a gain of ten
nanoseconds is under a tick: the numbers of `perf-compare.py` do not
move when the change is worth less than that. The report of the tree
has a clock of a millisecond -- its `elapsed_ms` -- and this is the
instrument for a change that small.

The number that this reads is the whole run of the script, and not only
the workload: the start of the tree is in it, and it is the same in
every run, so the difference between two numbers is the difference
between two changes.

The official is measured with the clock of the machine, in
`perf-compare.py`: its number is a number of ticks, and the error of it
is one tick.
"""

import json
import statistics
import subprocess
import shutil
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PERF = ROOT / "scripts" / "perf"
TREE = ROOT / "target" / "release" / "ahk.exe"
RUNS = 5


def the_workloads(wanted):
    """The workloads of the run."""
    if not wanted:
        return sorted(PERF.glob("*.ahk"))
    paths = []
    for name in wanted:
        path = PERF / (name if name.endswith(".ahk") else f"{name}.ahk")
        if not path.exists():
            raise SystemExit(f"there is no workload named {name} in {PERF}")
        paths.append(path)
    return paths


def measure(script):
    """The times of a workload under the tree, in milliseconds."""
    times = []
    with tempfile.TemporaryDirectory(ignore_cleanup_errors=True) as folder:
        work = Path(folder)
        # The workload is copied: the file that it writes is in the
        # folder of the script, and the folder of the repo is not the
        # place of a measurement.
        copy = work / script.name
        shutil.copyfile(script, copy)
        for _ in range(RUNS):
            process = subprocess.run(
                [str(TREE), "--dialect=v2", str(copy)],
                capture_output=True,
                text=True,
                timeout=300,
                cwd=str(work),
            )
            try:
                report = json.loads(process.stdout)
            except ValueError:
                raise SystemExit(
                    f"the tree did not answer for {script.name}:\n{process.stdout}\n{process.stderr}"
                )
            times.append(report.get("elapsed_ms"))
    times = [time for time in times if time is not None]
    if not times:
        raise SystemExit(f"the tree did not answer with a time for {script.name}")
    return times


def main():
    if not TREE.exists():
        raise SystemExit(
            f"the tree is not built: {TREE} (cargo build --release -p ahk-cli)"
        )
    for script in the_workloads(sys.argv[1:]):
        times = measure(script)
        print(f"{script.stem:<20}{times}  median {statistics.median(times)} ms")
    return 0


if __name__ == "__main__":
    sys.exit(main())
