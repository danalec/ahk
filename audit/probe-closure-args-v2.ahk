; The call of a closure with several parameters.

f := (a, b, c) => "a=" a " b=" b " c=" c
FileAppend(f("one", 2, 3), "*")

g := (a) => "a=" a
FileAppend(g("one"), "*")

h := (a, b) => "a=" a " b=" b
FileAppend(h("one", 2), "*")
ExitApp
