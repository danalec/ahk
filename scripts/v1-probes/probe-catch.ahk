out := ""
try
   throw "the message of the error"
catch e
   out .= "1) the what is [" e.What "] the extra is [" e.Extra "] the line is [" e.Line "]`n"
try
   throw "the second error"
catch, e2
   out .= "2) the what is [" e2.What "] the extra is [" e2.Extra "]`n"
out .= "the error level = " ErrorLevel "`n"
FileAppend, %out%, %A_ScriptDir%\probe-catch.txt
ExitApp
