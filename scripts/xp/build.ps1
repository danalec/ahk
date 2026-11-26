# Build the floor of the XP: a binary of thirty-two bits, for
# `i686-pc-windows-gnu`, without the standard library.
#
#   powershell -NoProfile -ExecutionPolicy Bypass scripts/xp/build.ps1
#   powershell -NoProfile -ExecutionPolicy Bypass scripts/xp/build.ps1 -Manifest scripts/xp/probe/Cargo.toml
#
# WHY: the floor of a binary is its import table, and the table of a
# binary of Rust is decided by three things: the target, the standard
# library it does or does not carry, and the C toolchain that links it.
# This script is the three of them, in one place, and it runs the gate
# of the table (`scripts/xp/gate.py`) over what it built.
#
# The three things, and why each one is needed:
#
#   1. The target is `i686-pc-windows-gnu`, and it is the one of the
#      plan of `docs/windows-legacy.md`: the CRT of the target is the
#      `msvcrt.dll` of Windows, the one that exists since NT, and there
#      is no api set in a binary of it.
#
#   2. The standard library does NOT come along: the `std` of Rust
#      imports `api-ms-win-core-synch-l1-2-0.dll` (`WaitOnAddress`,
#      Windows 8) and the Universal CRT, and the loader of XP refuses
#      the binary before the first instruction. A binary without `std`
#      asks for the XP-era symbols only.
#
#   3. The C toolchain is the MinGW of thirty-two bits with the
#      `msvcrt` runtime, and NOT `w64devkit`: the link line of the
#      target asks for `-lgcc_eh`, and `w64devkit` ships no
#      `libgcc_eh.a` (its `lib32` has the import libraries and
#      `libgcc.a`, but no unwinder). The link fails with
#      "cannot find -lgcc_eh" and the build stops.
#
# The toolchain of the compiler is a NIGHTLY with the source of the
# standard library, and not a stable one, and the reason is measured:
# the `core` that ships with the target is built with the unwinding on,
# and it asks for `rust_eh_personality` in its `.eh_frame`. A binary
# without `std` has no personality routine, and the link fails with
# "undefined reference to `rust_eh_personality`" as soon as the program
# touches the parts of `core` that carry the personality (the strings
# of UTF-8, for one). `-Z build-std=core,panic_abort` builds `core`
# again with the aborting panic, and the reference is gone.
#
# So the setup of the machine is:
#
#   rustup toolchain install nightly
#   rustup component add rust-src --toolchain nightly
#   rustup target add i686-pc-windows-gnu --toolchain nightly
#   # and the MinGW of thirty-two bits with the msvcrt runtime, in
#   # $env:AHK_XP_MINGW (the default is $HOME/tools/mingw32)
param(
    [string]$Manifest = "scripts/xp/probe/Cargo.toml",
    [string]$Target = "i686-pc-windows-gnu",
    [string]$Toolchain = "nightly",
    [string]$Mingw = $(if ($env:AHK_XP_MINGW) { $env:AHK_XP_MINGW } else { "$HOME\tools\mingw32" }),
    [string]$Stub = "$HOME\tools\gcc_eh_stub",
    [switch]$NoGate
)

$ErrorActionPreference = "Stop"

function Say([string]$text) { Write-Output $text }

if (-not (Test-Path $Manifest)) {
    Say "the manifest is not there: $Manifest"
    exit 1
}

# The linker of the target is the gcc of the architecture of the
# target, and it has to be the FIRST of the PATH: the `w64devkit` of
# the machine ships a gcc of sixty-four bits with the same names.
$arch = if ($Target -like "x86_64*") { "x86_64" } else { "i686" }
$gcc = Join-Path $Mingw "bin\$arch-w64-mingw32-gcc.exe"
if (-not (Test-Path $gcc)) {
    Say "the MinGW of the target is not there: $gcc"
    Say "thirty-two bits: the winlibs build 'i686 posix dwarf msvcrt';"
    Say "sixty-four bits: the w64devkit (see scripts/xp/README.md)"
    exit 1
}
$env:PATH = (Join-Path $Mingw "bin") + ";" + $env:PATH
Say "linker:   $gcc"

# The w64devkit ships no `libgcc_eh.a`: the link of the target asks
# for it, and the empty stub answers (the probe has no unwinder).
if ($arch -eq "x86_64" -and (Test-Path (Join-Path $Stub "libgcc_eh.a"))) {
    $stubdir = (Resolve-Path $Stub).Path -replace '\\','/'
    $env:RUSTFLAGS = "-C link-arg=-L$stubdir"
    Say "stub:     $stubdir/libgcc_eh.a"
}

# The target and the source of the standard library.
& rustup target add $Target --toolchain $Toolchain
if ($LASTEXITCODE -ne 0) { exit 1 }
& rustup component add rust-src --toolchain $Toolchain
if ($LASTEXITCODE -ne 0) { exit 1 }

Say "building: $Manifest for $Target"
# The messages of cargo are JSON here, because the place of the binary
# is in them: a manifest of its own has its own `target`, and a member
# of a workspace has the `target` of the workspace.
$messages = & cargo "+$Toolchain" build -Z build-std=core,panic_abort --release `
    --manifest-path $Manifest --target $Target --message-format=json
if ($LASTEXITCODE -ne 0) { exit 1 }

$binary = $null
foreach ($line in $messages) {
    if (-not $line) { continue }
    $message = $line | ConvertFrom-Json
    if ($message.reason -eq "compiler-artifact" -and $message.executable) {
        $binary = $message.executable
    }
}
if (-not $binary -or -not (Test-Path $binary)) {
    Say "the binary is not there: $binary"
    exit 1
}
Say "binary:   $binary"

if ($NoGate) { exit 0 }

# The gate of the floor: the table of the imports, and the question.
Say ""
& python "scripts/xp/gate.py" $binary
exit $LASTEXITCODE
