try {
  ComObjGet("")
} catch as e1 {
  FileAppend "get=[" e1.Message "]", "*"
}
try {
  ComObjActive("zzz")
} catch as e2 {
  FileAppend "active=[" e2.Message "]", "*"
}
t := ComObjType(5)
FileAppend "type=[" t "]", "*"
try {
  ComObjQuery(5)
} catch as e3 {
  FileAppend "query=[" e3.Message "]", "*"
}
try {
  ComObjFromPtr(0)
} catch as e4 {
  FileAppend "fromptr0=[" e4.Message "]", "*"
}
try {
  ComCall(1, 2, "bogus")
} catch as e5 {
  FileAppend "comcall=[" e5.Message "]", "*"
}
m := Map("a", 1)
p := ObjPtr(m)
back := ObjFromPtr(p)
same := back = m
FileAppend "roundtrip=[" same "] ptr=[" p "]", "*"
try {
  ObjRelease(1)
} catch as e6 {
  FileAppend "release=[" e6.Message "]", "*"
}
try {
  ObjPtrAddRef("x")
} catch as e7 {
  FileAppend "addref=[" e7.Message "]", "*"
}