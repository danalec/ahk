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
; The `GetModuleBaseAddr` of the collection of scripts of the v2 of jNizM
; (MIT), used as it is. The answer of it is the address of a module of the
; process, and the two engines are two processes: the case asks for the
; shape of the answer and not for the address.
GetModuleBaseAddr(ProcessID, ModuleName)
{
    static TH32CS_SNAPMODULE   := 0x00000008
    static TH32CS_SNAPMODULE32 := 0x00000010

    if !(hSnapshot := DllCall("CreateToolhelp32Snapshot", "UInt", TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, "UInt", ProcessID, "Ptr"))
        throw OSError()

    MODULEENTRY32 := Buffer(A_PtrSize = 8 ? 568 : 548)
    NumPut("UInt", MODULEENTRY32.Size, MODULEENTRY32, 0)
    if !(DllCall("Module32First", "Ptr", hSnapshot, "Ptr", MODULEENTRY32))
    {
        DllCall("CloseHandle", "Ptr", hSnapshot)
        throw OSError()
    }

    BaseAddr := Map()
    while (DllCall("Module32Next", "Ptr", hSnapshot, "Ptr", MODULEENTRY32))
    {
        Module := StrGet(MODULEENTRY32.Ptr + (A_PtrSize = 8 ? 48 : 32), 256, "cp0")
        if (Module = ModuleName)
        {
            BaseAddr["BaseAddr"] := Format("{:#016x}", NumGet(MODULEENTRY32, (A_PtrSize = 8 ? 24 : 20), "uptr"))
            BaseAddr["BaseSize"] := NumGet(MODULEENTRY32, (A_PtrSize = 8 ? 32 : 24), "uint")
        }
    }

    DllCall("CloseHandle", "Ptr", hSnapshot)
    return BaseAddr
}
try {
    ; kernel32 is mapped in every process of the machine.
    module := GetModuleBaseAddr(DllCall("kernel32\GetCurrentProcessId", "UInt"), "kernel32.dll")
    Emit("PT1=" Type(module) "|" module.Has("BaseAddr") "|" module.Has("BaseSize"))
    if module.Has("BaseAddr") {
        Emit("PT1shape=" Type(module["BaseAddr"]) "|" Type(module["BaseSize"]) "|" RegExMatch(module["BaseAddr"], "^0x[0-9a-f]{14}$") "|" (module["BaseSize"] > 0))
    }
    empty := GetModuleBaseAddr(DllCall("kernel32\GetCurrentProcessId", "UInt"), "no-such-module-here.dll")
    Emit("PT1empty=" Type(empty) "|" empty.Count)
} catch as e {
    Emit("PT1=ERR " Type(e) ": " e.Message)
}
