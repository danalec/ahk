AB := Class(Struct)
DefineProp(AB.Prototype, 'a', {type: Int8})
DefineProp(AB.Prototype, 'b', {type: Int32, pack: 1})
MsgBox AB().Size
