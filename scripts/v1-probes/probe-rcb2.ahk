; the callback of a method of a class is the method
out := ""
c := new MyClass()
answer := "NO"
if (c.MakeCallback())
   answer := "YES"
out .= "the callback of the method = " answer "`n"
answer := "NO"
if (c.MakeCallbackOfText())
   answer := "YES"
out .= "the callback of the qualified name = " answer "`n"
FileAppend, %out%, %A_ScriptDir%\probe-rcb2.txt
ExitApp

class MyClass {
   MakeCallback() {
      return RegisterCallback(this.OtherMethod)
   }
   MakeCallbackOfText() {
      return RegisterCallback("MyClass.OtherMethod")
   }
   OtherMethod(a, b) {
      return 1
   }
}
