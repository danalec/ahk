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
# The result is helper.exe, beside this script. The box is told where
# it is with the variable AHK_RAVEN_HELPER.

set -e
here=$(dirname "$0")
cd "$here"

llvm-dlltool -d kernel32.def -l libkernel32.a

clang --target=x86_64-w64-windows-gnu -fuse-ld=lld -O2 -nostdlib \
    -fno-stack-protector -fno-builtin \
    -Wl,--entry,raven_entry,--no-insert-timestamp \
    -o helper.exe helper.c -L. -lkernel32

echo "helper.exe is built."
