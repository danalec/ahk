; The arity of a function, as v2 reports it.

plain := (a, b) => 1
FileAppend("plain: min=" plain.MinParams " max=" plain.MaxParams " variadic=" plain.IsVariadic, "*")

rest := (a, b, c) => 1
FileAppend("three: min=" rest.MinParams " max=" rest.MaxParams " variadic=" rest.IsVariadic, "*")

withdefault := (a, b, c := 1) => 1
FileAppend("default: min=" withdefault.MinParams " max=" withdefault.MaxParams " variadic=" withdefault.IsVariadic, "*")

ExitApp
