#!/bin/sh
# The build of the helper: the guest program of the raven.
#
# The helper is a Windows program that the raven runs on the box. It is
# built the way the raven builds its own programs: no C runtime, the
# entry point of the PE called directly, and only the kernel32
# imported.
#
# The build needs no mingw and no header of the Windows: the import
# library is made by llvm-dlltool out of the definition of the five
# names that the helper asks for, and the program is linked by clang
# with lld.
#
#   sh crates/ahk-box/helper/build.sh
#
# The result is helper.exe, beside this script, and helper32.exe, the
# guest of the thirty-two bits. The box is told where they are with the
# variables AHK_RAVEN_HELPER and AHK_RAVEN_HELPER32.
#
# The two are built from the same source: the compiler of the C picks the
# path of the machine, and the thirty-two bits brings an assembly of its
# own -- the machine of thirty-two bits has six registers to spend, and
# the compiler of the C has none left for a block of assembly that wants
# three of them and takes three more.

set -e
here=$(dirname "$0")
cd "$here"

llvm-dlltool -d kernel32.def -l libkernel32.a
llvm-dlltool -d kernel32.def -l libkernel32_32.a -m i386

clang --target=x86_64-w64-windows-gnu -fuse-ld=lld -O2 -nostdlib \
    -fno-stack-protector -fno-builtin \
    -Wl,--entry,raven_entry,--no-insert-timestamp \
    -o helper.exe helper.c -L. -lkernel32

clang --target=i686-w64-windows-gnu -fuse-ld=lld -O2 -nostdlib \
    -fno-stack-protector -fno-builtin \
    -Wl,--entry,raven_entry,--no-insert-timestamp \
    -o helper32.exe helper.c helper32.s -L. -lkernel32_32

echo "helper.exe and helper32.exe are built."
