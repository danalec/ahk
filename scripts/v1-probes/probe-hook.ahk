; the hook of the messages of the thread, measured against the tree
;
; The answer of this probe is the file `probe-hook.txt` next to it: the
; callback that the script makes, the hook of the messages of the thread,
; and the message that the hook sees.

out := ""
seen := ""

cb := RegisterCallback("HookProc", "F")
made := "NO"
if (cb)
   made := "YES"
out .= "P1 the callback of a function of the script is made = " made "`n"

h := 0
if (cb)
   h := DllCall("SetWindowsHookEx", "Int", 4, "UPtr", cb, "UPtr", 0, "UInt", DllCall("GetCurrentThreadId"))
hooked := "NO"
if (h)
   hooked := "YES"
out .= "P2 the hook of the messages of the thread is set = " hooked "`n"

if (h)
{
   DllCall("SendMessageW", "UPtr", A_ScriptHwnd, "UInt", 0x4000, "UPtr", 0, "UPtr", 0)
   Sleep, 200
   DllCall("UnhookWindowsHookEx", "UPtr", h)
}
out .= "P3 the hook saw the message = " (seen ? "YES" : "NO") "`n"

FileAppend, %out%, %A_ScriptDir%\probe-hook.txt
ExitApp

HookProc(nCode, wParam, lParam) {
   global seen := "YES"
   return DllCall("user32\CallNextHookEx", "UPtr", 0, "Int", nCode, "UPtr", wParam, "UPtr", lParam)
}
