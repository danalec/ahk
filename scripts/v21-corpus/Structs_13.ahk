MsgBox DllCall("oleaut32\SysStringLen", BSTR, "abc" Chr(0) "123")
MsgBox DllCall("oleaut32\SysAllocString", "wstr", "xyz", BSTR)

struct BSTR {  ; This type can also be used in a struct.
    ptr : IntPtr
    size => DllCall("oleaut32\SysStringByteLen", "ptr", this, "uint")
    __value {
        get => StrGet(this)
        set {
            if this.ptr  ; In case of use in a struct.
                this.__delete()
            this.ptr := DllCall("oleaut32\SysAllocStringLen", "wstr", value, "uint", StrLen(value), "ptr")
        }
    }
    __delete() => DllCall("oleaut32\SysFreeString", "ptr", this)
}
