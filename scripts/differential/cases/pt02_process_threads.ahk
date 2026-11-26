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
; The `GetProcessThreads` of the collection of scripts of the v2 of jNizM
; (MIT), used as it is. The answer of it is the list of the threads of a
; process, and the two engines have a different number of them: the case
; asks for the shape of the answer.
GetProcessThreads(ProcessID)
{
    static TH32CS_SNAPTHREAD := 0x00000004

    if !(hSnapshot := DllCall("CreateToolhelp32Snapshot", "UInt", TH32CS_SNAPTHREAD, "UInt", ProcessID, "Ptr"))
        throw OSError()

    THREADENTRY32 := Buffer(28)
    NumPut("UInt", THREADENTRY32.Size, THREADENTRY32, 0)
    if !(DllCall("Thread32First", "Ptr", hSnapshot, "Ptr", THREADENTRY32))
    {
        DllCall("CloseHandle", "Ptr", hSnapshot)
        throw OSError()
    }

    Threads := Array()
    while (DllCall("Thread32Next", "Ptr", hSnapshot, "Ptr", THREADENTRY32))
    {
        OwnerProcessID := NumGet(THREADENTRY32, 12, "UInt")
        if (OwnerProcessID = ProcessID)
        {
            Threads.Push(NumGet(THREADENTRY32, 8, "UInt"))
        }
    }

    DllCall("CloseHandle", "Ptr", hSnapshot)
    return Threads
}
try {
    threads := GetProcessThreads(DllCall("kernel32\GetCurrentProcessId", "UInt"))
    Emit("PT2=" Type(threads) "|" (threads.Length > 0))
    if threads.Length {
        Emit("PT2item=" Type(threads[1]) "|" (threads[1] > 0))
    }
    none := GetProcessThreads(999999)
    Emit("PT2none=" Type(none) "|" none.Length)
} catch as e {
    Emit("PT2=ERR " Type(e) ": " e.Message)
}
