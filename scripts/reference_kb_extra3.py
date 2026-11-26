# SPDX-License-Identifier: Apache-2.0
"""Authored reference prose, batch 3: core + objects."""

E = {}
def add(n, d, s, r, x, rel=None, p=None):
    E[n] = {"desc": d, "syntax": s, "params": p or [], "returns": r, "example": x, "related": rel or []}

# -- the error classes -------------------------------------------------------
for n, d in [
    ("Error", "The base class of the errors of the language."),
    ("IndexError", "An index out of the bounds of a collection."),
    ("MemberError", "A member (property/method) that is not there."),
    ("MemoryError", "The machine is out of memory."),
    ("MethodError", "A method called on a value that has no such method."),
    ("OSError", "An error of the machine (a file, a socket, a call)."),
    ("PropertyError", "A property that cannot be read or written."),
    ("TargetError", "A window/control/pixel that was not found."),
    ("TimeoutError", "An operation that timed out."),
    ("TypeError", "A value of the wrong type."),
    ("UnsetError", "A variable that has not been assigned."),
    ("UnsetItemError", "An item of a collection that is unset."),
    ("ValueError", "A value that is invalid for the operation."),
    ("ZeroDivisionError", "A division by zero."),
]:
    add(n, d, "e := " + n + "(Message, What, Extra)", "An error object (Message/What/Extra/File/Line/Stack).", "throw " + n + "(\"x\")", ["Error", "Throw", "OnError"])
add("Any", "The root of the class tree; every value is an Any. It cannot be instantiated.", "Any", "The class; calling it answers 'no method named Call'.", "x := Any", ["Class", "Object"])
add("Class", "The class of classes; the base of every class object.", "Class", "The class object.", "C.Class", ["Any", "Object", "Prototype"])
add("Prototype", "The prototype object of a class (the instance base).", "C.Prototype", "The prototype.", "C.Prototype", ["Class", "Base"])
add("Extra", "The Extra property of an error (the value that failed).", "e.Extra", "The extra value.", "e.Extra", ["Error", "What"])
add("What", "The What property of an error (the name that failed).", "e.What", "The name.", "e.What", ["Error", "Extra"])
add("Stack", "The Stack property of an error (the call stack).", "e.Stack", "The stack text.", "e.Stack", ["Error", "Line"])
add("Line", "The Line property of an error (the line that failed).", "e.Line", "The line number.", "e.Line", ["Error", "File"])
add("Message", "The Message property of an error.", "e.Message", "The message.", "e.Message", ["Error"])
add("VarRef", "A reference to a variable (the &var operator).", "r := &var", "A VarRef with a name/scope.", "r := &x", ["IsSetRef", "Deref"])
add("Struct", "A view of memory at a place, of a type asked for (ks).", "s := Struct(Ptr, Type)", "A Struct with At/Size.", "s := Struct(p, \"Int\")", ["Buffer", "NumGet"])
add("Primitive", "The base of the primitive value types (ks).", "Primitive", "The class.", "x := Primitive", ["Any"])
add("MenuGetHandle", "The handle of a menu.", "H := MenuGetHandle(Menu)", "The handle.", "h := MenuGetHandle(m)", ["Menu", "MenuGetName"])
add("MenuGetName", "The name of a menu from its handle.", "S := MenuGetName(Handle)", "The name.", "n := MenuGetName(h)", ["Menu", "MenuGetHandle"])
add("IsActive", "Whether a thread/hotkey is active (ks).", "B := x.IsActive", "1/0.", "t.IsActive", ["Thread"])
add("IsInterruptible", "Whether the current thread is interruptible.", "B := IsInterruptible()", "1/0.", "IsInterruptible()", ["Critical", "Thread"])
add("IsComponentAvailable", "Whether a sound component is available (v1).", "B := IsComponentAvailable(Component)", "1/0.", "IsComponentAvailable(\"Volume\")", ["SoundGet"])

# -- the Is* / type tests ----------------------------------------------------
for n, d, x in [
    ("IsAlnum", "Whether the text is letters+digits.", 'IsAlnum("a1")'),
    ("IsAlpha", "Whether the text is letters.", 'IsAlpha("a")'),
    ("IsDigit", "Whether the text is digits.", 'IsDigit("1")'),
    ("IsFloat", "Whether the value is a float.", 'IsFloat(1.5)'),
    ("IsInteger", "Whether the value is an integer.", 'IsInteger(1)'),
    ("IsLower", "Whether the text is lower-case.", 'IsLower("a")'),
    ("IsUpper", "Whether the text is upper-case.", 'IsUpper("A")'),
    ("IsSpace", "Whether the text is whitespace.", 'IsSpace(" ")'),
    ("IsXDigit", "Whether the text is hex digits.", 'IsXDigit("ff")'),
    ("IsTime", "Whether the text is a valid time.", 'IsTime(t)'),
    ("IsFunc", "Whether the value is a function.", 'IsFunc(f)'),
    ("IsSetRef", "Whether a reference's variable is set.", 'IsSetRef(&x)'),
    ("IsByRef", "Whether a parameter is by-reference.", 'f.IsByRef(1)'),
    ("IsOptional", "Whether a parameter is optional.", 'f.IsOptional(1)'),
    ("IsVariadic", "Whether a function is variadic.", 'f.IsVariadic()'),
    ("IsBuiltIn", "Whether a function is built-in.", 'f.IsBuiltIn()'),
    ("IsClosure", "Whether a function is a closure.", 'f.IsClosure()'),
    ("IsMethod", "Whether a function is a method.", 'f.IsMethod()'),
]:
    add(n, d, "B := " + n + "(…)", "1 or 0.", x, ["IsNumber", "IsInteger"])
add("Number", "Converts a value to a number (integer or float).", "N := Number(Value)", "The number; throws TypeError if not convertible.", 'Number("3.5")', ["Integer", "Float", "IsNumber"])
add("Log", "The base-10 logarithm.", "N := Log(Number)", "The log.", "Log(100)  ; 2", ["Ln", "Exp"])
add("ACos", "The arc-cosine (radians).", "N := ACos(Number)", "The angle.", "ACos(1)  ; 0", ["Cos", "ASin"])
add("ASin", "The arc-sine (radians).", "N := ASin(Number)", "The angle.", "ASin(0)  ; 0", ["Sin", "ACos"])
add("ATan", "The arc-tangent (radians).", "N := ATan(Number)", "The angle.", "ATan(1)", ["Tan", "ATan2"])
add("StrCompare", "Compares two strings (case sense).", "N := StrCompare(A, B, CaseSense)", "<0/0/>0.", 'StrCompare("a", "b")', ["InStr", "Sort"])
add("StrPtr", "The pointer of a string's buffer.", "P := StrPtr(String)", "The pointer.", "p := StrPtr(s)", ["StrGet", "Buffer"])
add("StringCaseSense", "Sets the default string case sense (v1).", "StringCaseSense On|Off|Locale", "Nothing.", "StringCaseSense On", ["InStr", "StrCompare"])
add("StringGetPos", "The position of a substring (v1, 0-based).", "StringGetPos Out, Haystack, Needle", "Nothing; 0-based pos or -1.", "StringGetPos p, s, \"x\"", ["InStr"])
add("StringLeft", "The left N chars (v1).", "StringLeft Out, String, N", "Nothing.", "StringLeft o, s, 3", ["SubStr"])
add("StringLen", "The length (v1).", "StringLen Out, String", "Nothing.", "StringLen n, s", ["StrLen"])
add("StringLower", "Lower-case (v1).", "StringLower Out, String", "Nothing.", "StringLower o, s", ["StrLower"])
add("StringMid", "A substring (v1, 1-based).", "StringMid Out, String, Start, N", "Nothing.", "StringMid o, s, 2, 3", ["SubStr"])
add("StringReplace", "Replace (v1).", "StringReplace Out, String, Search, Replace", "Nothing.", "StringReplace o, s, \"a\", \"b\"", ["StrReplace"])
add("StringSplit", "Split (v1).", "StringSplit Out, String, Delim", "Nothing; Out0=count.", "StringSplit a, s, \",\"", ["StrSplit"])
add("StringTrimLeft", "Trim left N chars (v1).", "StringTrimLeft Out, String, N", "Nothing.", "StringTrimLeft o, s, 2", ["SubStr", "LTrim"])
add("VarSetCapacity", "Sets a variable's buffer capacity (v1).", "VarSetCapacity Var, Bytes", "Nothing.", "VarSetCapacity v, 16", ["VarSetStrCapacity", "Buffer"])
add("ComObjArray", "Creates a COM safe array.", "A := ComObjArray(Type, Counts...)", "A COM array.", "a := ComObjArray(0xB, 3)", ["ComObject", "ComValue"])
add("ComObjCreate", "Creates a COM object (v1).", "Obj := ComObjCreate(CLSID)", "The object.", "o := ComObjCreate(\"Shell.Application\")", ["ComObject"])
add("ComObjError", "Sets whether COM errors throw (v1).", "ComObjError On|Off", "Nothing.", "ComObjError Off", ["ComObject", "OnError"])
add("ObjCreateWithDataPtr", "Creates an object over a data pointer (ks).", "O := ObjCreateWithDataPtr(Ptr, Size)", "The object.", "o := ObjCreateWithDataPtr(p, n)", ["ObjGetDataPtr", "Struct"])
add("ObjGetDataPtr", "The data pointer of an object (ks).", "P := ObjGetDataPtr(Obj)", "The pointer.", "p := ObjGetDataPtr(o)", ["ObjCreateWithDataPtr", "ObjGetDataSize"])
add("ObjGetDataSize", "The data size of an object (ks).", "N := ObjGetDataSize(Obj)", "The size.", "n := ObjGetDataSize(o)", ["ObjGetDataPtr"])
add("ObjSetDataPtr", "Sets the data pointer of an object (ks).", "ObjSetDataPtr Obj, Ptr, Size", "Nothing.", "ObjSetDataPtr o, p, n", ["ObjGetDataPtr"])
add("__Init", "The initializer of a class (called on construction).", "C.__Init(this, Args...)", "Nothing.", "class C { __Init() {} }", ["__New", "Class"])

# -- object / array / map / func methods -------------------------------------
add("Base", "The base object of a value.", "B := Obj.Base", "The base; assigning sets it.", "o.Base", ["ObjSetBase", "Class"])
add("Bind", "Binds arguments to a function, answering a BoundFunc.", "F := Func.Bind(Args...)", "A BoundFunc.", "f := Func.Bind(1)", ["BoundFunc", "ObjBindMethod"])
add("BoundFunc", "A function with bound arguments/this.", "F := Func.Bind(...)", "The BoundFunc object.", "f.Bind(1)", ["Func", "Bind"])
add("Call", "Calls a function object.", "R := Func.Call(Args...)", "The return.", "f.Call(1)", ["Func", "BoundFunc"])
add("Capacity", "The capacity of an Array/Map.", "N := Arr.Capacity", "The capacity; assigning sets it.", "a.Capacity", ["Array", "Length"])
add("CaseSense", "The case-sensitivity of a Map.", "M := Map.CaseSense", "On/Off/Locale; assigning sets it.", "m.CaseSense := \"On\"", ["Map"])
add("Clear", "Removes all items of a Map/Array.", "Obj.Clear()", "Nothing.", "m.Clear()", ["Map", "Array", "Delete"])
add("Clone", "A shallow copy of an object/array.", "C := Obj.Clone()", "The copy.", "c := a.Clone()", ["Array", "Object"])
add("Closure", "A function that captures its enclosing scope.", "F := (*) => x", "The closure.", "f := (*) => x", ["Func", "IsClosure"])
add("Contains", "Whether a collection contains a value (ks).", "B := Arr.Contains(Value)", "1/0.", "a.Contains(2)", ["IndexOf", "Array"])
add("Count", "The number of items of a Map/Array.", "N := Obj.Count", "The count.", "m.Count", ["Map", "Length"])
add("DeleteProp", "Deletes a property of an object.", "Obj.DeleteProp(Name)", "The deleted value.", "o.DeleteProp(\"x\")", ["DefineProp", "HasProp"])
add("Filter", "Filters a collection by a predicate (ks).", "A := Arr.Filter(Pred)", "The filtered Array.", "a.Filter((x) => x > 1)", ["Array", "Map", "Join"])
add("FindIndex", "The index of the first item matching a predicate (ks).", "N := Arr.FindIndex(Pred)", "The index or 0.", "a.FindIndex((x) => x = 2)", ["IndexOf", "Array"])
add("Get", "Reads an item of a Map/Array.", "V := Obj.Get(Key, Default)", "The value or Default.", "m.Get(\"k\", 0)", ["Set", "Map", "Array"])
add("GetOwnPropDesc", "The descriptor of an own property.", "D := Obj.GetOwnPropDesc(Name)", "The descriptor object.", "o.GetOwnPropDesc(\"x\")", ["DefineProp", "OwnProps"])
add("Has", "Whether a Map has a key.", "B := Map.Has(Key)", "1/0.", "m.Has(\"k\")", ["Map", "Get"])
add("HasOwnProp", "Whether an object has an own property.", "B := Obj.HasOwnProp(Name)", "1/0.", "o.HasOwnProp(\"x\")", ["HasProp", "OwnProps"])
add("IndexOf", "The index of a value in an Array (ks).", "N := Arr.IndexOf(Value)", "The index or 0.", "a.IndexOf(2)", ["Contains", "FindIndex"])
add("InsertAt", "Inserts items into an Array at a position.", "Arr.InsertAt(Pos, Values...)", "Nothing.", "a.InsertAt(2, 9)", ["Array", "RemoveAt", "Push"])
add("Join", "Joins a collection into a string (ks).", "S := Arr.Join(Sep)", "The string.", "a.Join(\",\")", ["StrSplit", "Array"])
add("MaxParams", "The max parameters of a function.", "N := Func.MaxParams", "The number.", "f.MaxParams", ["MinParams", "IsVariadic"])
add("MinParams", "The min parameters of a function.", "N := Func.MinParams", "The number.", "f.MinParams", ["MaxParams", "IsOptional"])
add("ObjAddRef", "Adds a reference to an object.", "ObjAddRef Obj", "Nothing.", "ObjAddRef o", ["ObjRelease", "ObjPtr"])
add("ObjGetCapacity", "The capacity of an object's buffer (v1).", "N := ObjGetCapacity(Obj)", "The capacity.", "ObjGetCapacity(o)", ["ObjSetCapacity"])
add("ObjHasOwnProp", "Whether an object has an own property (v1).", "B := ObjHasOwnProp(Obj, Name)", "1/0.", "ObjHasOwnProp(o, \"x\")", ["HasOwnProp"])
add("ObjOwnPropCount", "The number of own properties (v1).", "N := ObjOwnPropCount(Obj)", "The count.", "ObjOwnPropCount(o)", ["OwnProps"])
add("ObjOwnProps", "An Array of the own property names (v1).", "A := ObjOwnProps(Obj)", "The Array.", "ObjOwnProps(o)", ["OwnProps"])
add("ObjSetBase", "Sets the base of an object.", "ObjSetBase Obj, Base", "Nothing.", "ObjSetBase o, b", ["Base", "ObjGetBase"])
add("ObjSetCapacity", "Sets an object's buffer capacity (v1).", "ObjSetCapacity Obj, Bytes", "Nothing.", "ObjSetCapacity o, 16", ["ObjGetCapacity"])
add("OwnProps", "Iterates the own properties of an object.", "for name, value in Obj.OwnProps()", "The enumerator.", "for n, v in o.OwnProps()", ["HasOwnProp", "DefineProp"])
add("Params", "The parameter count/info of a function (ks).", "N := Func.Params", "The count.", "f.Params", ["MinParams", "MaxParams"])
add("Pop", "Removes and answers the last item of an Array.", "V := Arr.Pop()", "The item.", "v := a.Pop()", ["Array", "Push"])
add("Push", "Appends items to an Array.", "Arr.Push(Values...)", "Nothing.", "a.Push(1)", ["Array", "Pop"])
add("RegExMatchInfo", "The match object of RegExMatch (Pos/Len/Value/...).", "M := RegExMatch(...)", "The match object.", "m[0]", ["RegExMatch", "RegExReplace"])
add("Remove", "Removes a key/item from a Map/Array (ks).", "Obj.Remove(Key)", "The removed value.", "m.Remove(\"k\")", ["Delete", "Map"])
add("RemoveAt", "Removes items of an Array at a position.", "Arr.RemoveAt(Pos, N)", "Nothing.", "a.RemoveAt(2)", ["Array", "InsertAt"])
add("Set", "Writes an item of a Map/Array.", "Obj.Set(Key, Value)", "Nothing.", "m.Set(\"k\", 1)", ["Get", "Map"])
add("ToString", "The text of a collection (ks).", "S := Arr.ToString()", "The string.", "a.ToString()", ["Join", "String"])
add("EndsWith", "Whether a string ends with a suffix (ks).", "B := Str.EndsWith(Suffix)", "1/0.", "s.EndsWith(\"x\")", ["StartsWith", "SubStr"])
add("StartsWith", "Whether a string starts with a prefix (ks).", "B := Str.StartsWith(Prefix)", "1/0.", "s.StartsWith(\"a\")", ["EndsWith", "SubStr"])

EXTRA3 = E
