class AB1 {
    #StructPack 1
    a : Int8
    b : Int32
}
class AB2 {
    a : Int8
    b : Int32
}
MsgBox "AB1: " AB1.Prototype.GetOwnPropDesc('b').Offset
MsgBox "AB2: " AB2.Prototype.GetOwnPropDesc('b').Offset
