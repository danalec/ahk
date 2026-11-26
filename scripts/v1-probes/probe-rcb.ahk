out := ""
try {
   fn := RegisterCallback("")
   out .= "the callback of an empty name = " fn "`n"
} catch e {
   out .= "the empty name failed: " e.What "`n"
}
try {
   fn2 := RegisterCallback("NoSuchFunctionHere")
   out .= "the callback of a name that is not there = " fn2 "`n"
} catch e2 {
   out .= "the name that is not there failed: " e2.What "`n"
}
FileAppend, %out%, %A_ScriptDir%\probe-rcb.txt
ExitApp
