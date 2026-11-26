#ErrorStdOut
#Warn All, StdOut
OnError(__backstop)
__backstop(e, mode) {
    FileAppend("UNCAUGHT|" Type(e) "|" e.Message "`n", "*")
    return true
}
Emit(s) {
    FileAppend(s "`n", "*")
}
; The `IsProcessElevated` of the collection of scripts of the v2 of jNizM
; (MIT), used as it is. The answer of it is the elevation of the token of
; the process, and the two engines run with the same token.
IsProcessElevated(ProcessID)
{
    static INVALID_HANDLE_VALUE              := -1
    static PROCESS_QUERY_INFORMATION         := 0x0400
    static PROCESS_QUERY_LIMITED_INFORMATION := 0x1000
    static TOKEN_QUERY                       := 0x0008
    static TOKEN_QUERY_SOURCE                := 0x0010
    static TokenElevation                    := 20

    hProcess := DllCall("OpenProcess", "UInt", PROCESS_QUERY_INFORMATION, "Int", False, "UInt", ProcessID, "Ptr")
    if (!hProcess) || (hProcess = INVALID_HANDLE_VALUE)
    {
        hProcess := DllCall("OpenProcess", "UInt", PROCESS_QUERY_LIMITED_INFORMATION, "Int", False, "UInt", ProcessID, "Ptr")
        if (!hProcess) || (hProcess = INVALID_HANDLE_VALUE)
            throw OSError()
    }

    if !(DllCall("advapi32\OpenProcessToken", "Ptr", hProcess, "UInt", TOKEN_QUERY | TOKEN_QUERY_SOURCE, "Ptr*", &hToken := 0))
    {
        DllCall("CloseHandle", "Ptr", hProcess)
        throw OSError()
    }

    if !(DllCall("advapi32\GetTokenInformation", "Ptr", hToken, "Int", TokenElevation, "UInt*", &IsElevated := 0, "UInt", 4, "UInt*", &Size := 0))
    {
        DllCall("CloseHandle", "Ptr", hToken)
        DllCall("CloseHandle", "Ptr", hProcess)
        throw OSError()
    }

    DllCall("CloseHandle", "Ptr", hToken)
    DllCall("CloseHandle", "Ptr", hProcess)
    return IsElevated
}
try {
    ; The token of the process of the engine: the two engines run with the
    ; same one, and the answer of it is a number.
    Emit("PT4=" Type(IsProcessElevated(DllCall("kernel32\GetCurrentProcessId", "UInt"))) "|" (IsProcessElevated(DllCall("kernel32\GetCurrentProcessId", "UInt")) ? 1 : 0))
} catch as e {
    Emit("PT4=ERR " Type(e) ": " e.Message)
}
try {
    ; The process that is not there: the v1 throws the error of the machine.
    IsProcessElevated(999999)
    Emit("PT4none=NO ERROR")
} catch as e {
    Emit("PT4none=" Type(e) ": " e.Message)
}
