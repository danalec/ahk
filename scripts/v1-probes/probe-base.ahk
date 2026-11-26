; The base of an instance is the class of it, and the statics of the
; classes of the chain are statics of the class.
;
; Measured on the binary of AutoHotkey v1.1.37.02: the `This.Base.X` of a
; method of a class that extends another is the `Static X` of the class
; that it extends, and the `This.X` of it is the same. The `Static Y` of
; the class itself is read the same way.
;
; The answer of the probe is the four values: they are the same in the v1
; and in the tree.

Class A {
   Static X := "A-static"
}

Class B extends A {
   Static Y := "B-static"

   __New() {
      global out
      a := This.Base.X
      b := This.Base.Y
      c := This.X
      d := This.Y
      out := "base.X=" a "|base.Y=" b "|this.X=" c "|this.Y=" d "`n"
   }
}

instance := new B
FileAppend, %out%, %A_ScriptDir%\probe-base.txt
ExitApp
