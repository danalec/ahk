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
; The `GetThreadStartAddr` of the collection of scripts of the v2 of jNizM
; (MIT), used as it is. The answer of it is the address of the start of
; every thread of a process, and the two engines have different ones: the
; case asks for the shape of the answer.
GetThreadStartAddr(ProcessID)
{
    #DllLoad "ntdll.dll"

    static TH32CS_SNAPTHREAD               := 0x00000004
    static THREAD_QUERY_INFORMATION        := 0x0040
    static ThreadQuerySetWin32StartAddress := 9

    if !(hSnapshot := DllCall("CreateToolhelp32Snapshot", "UInt", TH32CS_SNAPTHREAD, "UInt", ProcessID, "Ptr"))
        throw OSError()

    THREADENTRY32 := Buffer(28)
    NumPut("UInt", THREADENTRY32.Size, THREADENTRY32, 0)
    if !(DllCall("Thread32First", "Ptr", hSnapshot, "Ptr", THREADENTRY32))
    {
        DllCall("CloseHandle", "Ptr", hSnapshot)
        throw OSError()
    }

    StartAddr := Map()
    while (DllCall("Thread32Next", "Ptr", hSnapshot, "Ptr", THREADENTRY32))
    {
        OwnerProcessID := NumGet(THREADENTRY32, 12, "UInt")
        if (OwnerProcessID = ProcessID)
        {
            ThreadID := NumGet(THREADENTRY32, 8, "UInt")
            if !(hThread := DllCall("OpenThread", "UInt", THREAD_QUERY_INFORMATION, "Int", false, "UInt", ThreadID, "Ptr"))
                continue
            if (DllCall("ntdll\NtQueryInformationThread", "Ptr", hThread, "UInt", ThreadQuerySetWin32StartAddress, "Ptr*", &ThreadStartAddr := 0, "UInt", A_PtrSize, "UInt*", 0) != 0)
                continue
            StartAddr[ThreadID] := Format("{:#016x}", ThreadStartAddr)
            DllCall("CloseHandle", "Ptr", hThread)
        }
    }

    DllCall("CloseHandle", "Ptr", hSnapshot)
    return StartAddr
}
try {
    threads := GetThreadStartAddr(DllCall("kernel32\GetCurrentProcessId", "UInt"))
    Emit("PT3=" Type(threads) "|" (threads.Count > 0))
    for tid, address in threads {
        Emit("PT3shape=" Type(address) "|" RegExMatch(address, "^0x[0-9a-f]{14}$") "|" (tid > 0))
        break
    }
} catch as e {
    Emit("PT3=ERR " Type(e) ": " e.Message)
}
