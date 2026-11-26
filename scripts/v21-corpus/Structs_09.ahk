struct X {
    y : Int32[10]
}
z := X()
MsgBox z.Size ; 40
MsgBox z.y.Size ; 40
MsgBox z.y.Length ; 10
MsgBox z.y is Int32[10] ; true
MsgBox z.y is Int32[100] ; false

struct XX {
    items : X[2]
}
zz := XX()
MsgBox zz.Size ; 80
