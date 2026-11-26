@echo off
rem The guest-side driver of the XP behaviour measurement.
rem
rem   driver.bat
rem
rem The corpus is at c:\nb (v1\ and v2\), the runner is c:\ahk.exe and
rem the reference v1 binary c:\ref\AutoHotkeyU32.exe. Each case runs in
rem its own process; the answers are appended, with a ===case=== marker
rem per case, to c:\res\v1t.txt (tree v1), v1r.txt / v1r.err (reference
rem stdout / stderr), v1code.txt (reference exit codes) and v2t.txt
rem (tree v2). The host reads c:\res back (FTP or floppy) and compares.
rem
rem The working directory is pinned to c:\ so relative-path cases do
rem not depend on the caller's cwd (the XP profile dir reads with
rem OS error 87 and is not a stable base).
cd /d c:\
mkdir c:\res 2>nul
for %%f in (c:\nb\v1\*.ahk) do call :v1 %%~nf
for %%f in (c:\nb\v2\*.ahk) do call :v2 %%~nf
echo DONE > c:\res\done.txt
goto :eof

:v1
echo ===%1=== >> c:\res\v1t.txt
c:\ahk.exe --dialect=v1 c:\nb\v1\%1.ahk >> c:\res\v1t.txt 2>nul
echo ===%1=== >> c:\res\v1r.txt
c:\ref\AutoHotkeyU32.exe /ErrorStdOut c:\nb\v1\%1.ahk >> c:\res\v1r.txt 2>> c:\res\v1r.err
echo %1 %ERRORLEVEL% >> c:\res\v1code.txt
goto :eof

:v2
echo ===%1=== >> c:\res\v2t.txt
c:\ahk.exe c:\nb\v2\%1.ahk >> c:\res\v2t.txt 2>nul
goto :eof
