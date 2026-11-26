# Build the runner of the tree for the floor of Windows XP, and gate it.
#
#   powershell -NoProfile -ExecutionPolicy Bypass scripts/xp/release.ps1
#   powershell ... scripts/xp/release.ps1 -Target x86_64-win7-windows-gnu
#
# The binary of the XP floor is not the binary of the modern Windows:
# it is the tier-3 target `*-win7-windows-gnu` with the standard
# library emended for the era of XP (`patch_std.py`), built with
# `-Z build-std` on a PINNED nightly, and linked by the MinGW of the
# `msvcrt` flavour. The gate (`gate.py`) reads the import table over
# the measured floor and fails if the binary asks for anything the XP
# machines do not have.
#
# The toolchain is pinned: the emends of `patch_std.py` anchor to the
# text of the `std`, and a floating nightly breaks them. The pin is
# the nightly the floor was measured with.
#
# The MinGW: the thirty-two bits is the winlibs build of the `msvcrt`
# runtime (it ships `libgcc_eh.a`, which the link asks for); the
# sixty-four bits is the `w64devkit` with the empty stub of
# `libgcc_eh` (the w64devkit ships none). When the tools are not on
# the machine, the script downloads them; `-Mingw` names a toolchain
# that is already there.

param(
    [string]$Target = "i686-win7-windows-gnu",
    [string]$Toolchain = "nightly-2026-11-13",
    [string]$Mingw = $env:AHK_XP_MINGW,
    [string]$Tools = "$HOME\tools",
    [switch]$NoGate
)

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
Set-Location $root

function Say([string]$text) { Write-Output $text }

$arch = if ($Target -like "x86_64*") { "x86_64" } else { "i686" }

# --- the toolchain -----------------------------------------------------------
& rustup toolchain install $Toolchain --profile minimal
if ($LASTEXITCODE -ne 0) { exit 1 }
& rustup component add rust-src --toolchain $Toolchain
if ($LASTEXITCODE -ne 0) { exit 1 }
Say "toolchain: $Toolchain"

# --- the MinGW ---------------------------------------------------------------
if (-not $Mingw) {
    if ($arch -eq "i686") {
        $Mingw = Join-Path $Tools "mingw32"
        if (-not (Test-Path (Join-Path $Mingw "bin\i686-w64-mingw32-gcc.exe"))) {
            Say "downloading the winlibs MinGW of thirty-two bits (msvcrt)"
            $url = "https://github.com/brechtsanders/winlibs_mingw/releases/download/16.2.0posix-23.1.3-14.0.0-msvcrt-r3/winlibs-i686-posix-dwarf-gcc-16.2.0-llvm-23.1.3-mingw-w64msvcrt-14.0.0-r3.7z"
            $archive = Join-Path $env:TEMP "winlibs-i686-msvcrt.7z"
            Invoke-WebRequest -Uri $url -OutFile $archive
            New-Item -ItemType Directory -Force -Path $Tools | Out-Null
            & 7z x -y "-o$Tools" $archive | Out-Null
        }
    } else {
        $Mingw = Join-Path $Tools "w64devkit"
        if (-not (Test-Path (Join-Path $Mingw "bin\x86_64-w64-mingw32-gcc.exe"))) {
            Say "downloading the w64devkit of sixty-four bits"
            $url = "https://github.com/skeeto/w64devkit/releases/download/v2.11.0/w64devkit-x64-2.11.0.7z.exe"
            $archive = Join-Path $env:TEMP "w64devkit.7z.exe"
            Invoke-WebRequest -Uri $url -OutFile $archive
            New-Item -ItemType Directory -Force -Path $Tools | Out-Null
            & 7z x -y "-o$Tools" $archive | Out-Null
        }
    }
}
$gcc = Join-Path $Mingw "bin\$arch-w64-mingw32-gcc.exe"
if (-not (Test-Path $gcc)) {
    Say "the MinGW of the target is not there: $gcc"
    exit 1
}
$env:PATH = (Join-Path $Mingw "bin") + ";" + $env:PATH
Say "linker:    $gcc"

# The empty stub of libgcc_eh for the sixty-four bits: the w64devkit
# ships none, and the link of the target asks for it. An empty archive
# answers (the binary of panic=abort has no unwinder).
$rustflags = "-C strip=symbols -C opt-level=z"
if ($arch -eq "x86_64") {
    $stub = Join-Path $Tools "gcc_eh_stub"
    New-Item -ItemType Directory -Force -Path $stub | Out-Null
    $stubfile = Join-Path $stub "libgcc_eh.a"
    if (-not (Test-Path $stubfile)) {
        [System.IO.File]::WriteAllBytes($stubfile, [System.Text.Encoding]::ASCII.GetBytes("!<arch>`n"))
    }
    $stubdir = (Resolve-Path $stub).Path -replace '\\','/'
    $rustflags += " -C link-arg=-L$stubdir"
}

# --- the emend of the std ----------------------------------------------------
$env:AHK_XP_TOOLCHAIN = "$Toolchain-x86_64-pc-windows-msvc"
& python scripts/xp/patch_std.py
if ($LASTEXITCODE -ne 0) { exit 1 }

# The fingerprint of the cargo does not change when the std changes:
# the build folder of the target has to be erased for the emend to
# enter the binary.
$builddir = "target/$Target/release/build"
if (Test-Path $builddir) { Remove-Item -Recurse -Force $builddir }

# --- the build ---------------------------------------------------------------
Say "building:  ahk-cli for $Target"
$env:RUSTFLAGS = $rustflags
& cargo "+$Toolchain" build -Z build-std=std,core,alloc,panic_abort `
    -p ahk-cli --features xp --target $Target --release
if ($LASTEXITCODE -ne 0) { exit 1 }

$binary = "target/$Target/release/ahk.exe"
if (-not (Test-Path $binary)) {
    Say "the binary is not there: $binary"
    exit 1
}
Say "binary:    $binary"

if ($NoGate) { exit 0 }

# --- the gate ----------------------------------------------------------------
& python scripts/xp/gate.py $binary
exit $LASTEXITCODE
