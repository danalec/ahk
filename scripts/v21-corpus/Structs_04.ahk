POINT := Class(Struct)  ; v2.1-alpha.21 and older used Object instead of Struct.
DefineProp(POINT.Prototype, 'x', {type: Int32})
DefineProp(POINT.Prototype, 'y', {type: Int32})
pt := POINT()
