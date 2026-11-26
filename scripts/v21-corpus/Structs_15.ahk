struct LoadedHModule {
    h : IntPtr
    __value => this.h ? this : throw(OSError(A_LastError, -1))
    __delete() => DllCall("FreeLibrary", "ptr", this.h)
}

check
gdip := DllCall("LoadLibrary", "str", "gdiplus", LoadedHModule)
check
gdip := unset
check

DllCall("LoadLibrary", "str", "this is bound to fail", LoadedHModule) ; OSError(126)

; By contrast with LoadLibrary, a zero return value from GetModuleHandle
; is expected, and a non-zero handle should not be freed automatically.
check() => MsgBox(DllCall("GetModuleHandle", "str", "gdiplus", "ptr") ? "Loaded" : "Not loaded")
