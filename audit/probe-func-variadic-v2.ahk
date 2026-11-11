; The arity of a variadic function, as v2 reports it.

withrest := (a, b, c, Rest*) => 1
FileAppend("with a rest: min=" withrest.MinParams " max=" withrest.MaxParams " variadic=" withrest.IsVariadic, "*")

onlyrest := (Rest*) => 1
FileAppend("only a rest: min=" onlyrest.MinParams " max=" onlyrest.MaxParams " variadic=" onlyrest.IsVariadic, "*")

ExitApp
