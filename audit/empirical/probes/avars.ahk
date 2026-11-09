; Probe: a representative set of the built-in variables of v2. Each one is
; read inside a try, so the ones the tree refuses are recorded with the
; exact reason instead of stopping the probe.
acc := ""
names := ["A_AhkVersion", "A_ScreenWidth", "A_ScreenHeight", "A_ComputerName", "A_UserName", "A_OSVersion", "A_Now", "A_Year", "A_TickCount", "A_ScriptDir", "A_ScriptName", "A_WorkingDir", "A_Temp", "A_Space", "A_Tab", "A_LineNumber", "A_PtrSize", "A_IsAdmin", "A_Language", "A_Args", "A_KeyDelay", "A_CoordModeMouse", "A_ThisFunc", "A_ScriptHwnd", "A_ExitReason", "A_Clipboard"]

for name in names {
    try {
        v := %name%
        acc := acc name "=OK|type:" Type(v) "|[" SubStr("" v, 1, 40) "]`n"
    } catch as e {
        acc := acc name "=ERR|" e.Message "`n"
    }
}

FileAppend acc, "*"
