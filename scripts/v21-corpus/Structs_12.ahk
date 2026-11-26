; Define a reusable "meta-class" or "generic class" for C-style strings.
; Calling CString itself returns a new class for use in a type expression.
CString(n, cp:="UTF-16") {
    c := Class(Struct)
    unitSize := StrPut("", cp)  ; Size in bytes of a single "character".
    DefineProp(c.Prototype, '_chars', {type: unitSize=2 ? UInt16[n] : Int8[n]})
    DefineProp(c.Prototype, '__value', {
        get: (this)        => StrGet(this,, cp),
        set: (this, value) => StrPut(value, this,, cp)
    })
    return c
}

; Define a struct containing a string of at most 32 UTF-8 code units.
struct XStruct {
  str : CString(32, "UTF-8")
}

x := XStruct()
x.str := "Hello!"
MsgBox x.str
