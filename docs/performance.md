# The performance of the tree, and where the time goes

The tree is a re-implementation of AutoHotkey in Rust, and its speed is
measured against the binary of the official release: the v2.0.32
(`ahk-v2-headless`) and the v1.1.37.02. This document says how the
measurements are made, what they say today, and what the next steps are.

## The instrument

Two kinds of instrument, and they answer different questions.

**The workloads of the repo** (`scripts/perf/`) and the harness that runs
them (`scripts/perf-compare.py`) say whether the tree is faster or slower
than the official on the work that a script does. The harness runs every
workload under the tree and under the official v2.0.32, and prints the
median of five runs of each:

```
python scripts/perf-compare.py            # every workload
python scripts/perf-compare.py method_call   # one of them
```

**The focused workload** is one workload of five million turns, alone. The noise of the big suite is about ±5%, and the
noise of the clock is 15,6 ms; a focused workload of five million turns
puts the signal above both. It is the instrument of the *cost of a path*:
one call, one property, one literal.

Every number below is the median of three runs of the release build, and
the official is measured with the same script.

## The cost of a path, today

| The workload | The tree | The official v2.0.32 | The ratio |
|---|---:|---:|---:|
| `arr_push_get` | 109 ms | 218 ms | 0,50x |
| `func_call` | 93 ms | 109 ms | 0,85x |
| `map_set_get` | 219 ms | 313 ms | 0,70x |
| `method_call` | 78 ms | 94 ms | 0,83x |
| `noop_loop` | 16 ms | 31 ms | 0,52x |
| `numput_numget` | 47 ms | 140 ms | 0,34x |
| `obj_prop` | 31 ms | 63 ms | 0,49x |
| `strings` | 78 ms | 63 ms | 1,24x |
| `while_loop` | 172 ms | 250 ms | 0,69x |

The tree is faster in eight of the nine workloads (`numput_numget` dropped
from 125-141 ms to 47 ms after caching the buffer cell in compiled `NumPut`
and decoding via `from_le_bytes` in commit `8122aa99`), with only `strings`
sitting one 15,6 ms tick behind the official.

The clock of the machine counts in 15,6 milliseconds, and every number
above is a number of ticks: a workload under a hundred milliseconds is
measured in one or two ticks, and the ratio of the fast ones is coarse.

**The clock of the harness is too coarse for the small wins.** A gain of
ten nanoseconds is under a tick, and the numbers do not move. The report
of the tree has a clock of a millisecond -- its `elapsed_ms` ✓ -- and
`scripts/perf-fine.py` is the instrument that reads it:

```
python scripts/perf-fine.py numput_numget
```

The `NumPut` of a loop is 154, 153, 152, 151 and 153 milliseconds with
the match of the type of a number by the length of its name, and 154,
150, 151, 157 and 151 without it -- the same number, and the change is
not a win.

The workloads that the tables below carry lived in `target/tmp`, and they
are gone with it: the table above is the one that the repo carries, and
`scripts/perf-compare.py` is what measures it.

Every number is the median of seven runs of the release build of the tree
and of the official v2.0.32, on the machine of the owner. The "before"
is the tree of the release 0.6.6, before the three levers of the text,
of the names and of the loop.

The tree is faster than the official in three of the five, and the fourth
is a tie. The loop of the integers is the one that got slower: 78 ms
before, 93 ms now, where the official is at 78 ms.

The other paths of the same instrument:

| The workload | The tree | The official v2.0.32 |
|---|---:|---:|
| `s := InStr(hay, "b")` (five hundred thousand turns) | 32 ms | 31 ms |
| `c.Inc(1)` (three hundred thousand turns) | 56 ms | 62 ms |
| `o.a := o.a + o.b` (five million turns) | 250 ms | 375 ms |

The property of an object is a third faster than the official, and the
call of a method is at the time of the official now.

The first line is the important one: **the compiled path of the tree is as
fast as the official.** The tree compiles the loops of integers, of floats
and of registers onto flat slots (`Runtime::compile_fast_loop`), and a
loop that it compiles answers in the time of the official.

The call of a built-in is in the compiled path too (the commit
`07ac318b`): the row of the built-in is resolved when the loop is compiled,
and the arguments are the ones of the loop. A call whose name is shadowed
is not compiled -- a function of the script with that name wins over the
built-in, and a class type of the runtime with that name wins over it too.

The call of a **method** of a class of the script is in the compiled path:
the name of the method is the text of the source, and it is read where it
is written; the target of the call is the value of an expression of the
loop; and the arguments are the ones that read nothing -- the loop keeps
its variables in the registers, and the value of a variable of the loop is
not in the script while the loop runs. Measured with the fine clock: the
`method_call` answers in 56 ms where it took 104 ms, and the official
takes 62 ms.

The call of a **function** of the script is in the compiled path too, and
its arguments are read from the registers of the loop: the value of a
variable of the loop is not in the script while the loop runs, and the
generic path reads the script. Measured with the fine clock: the
`func_call` answers in 73 ms where it took 131 ms, and the official takes
78 ms; the phase `phase_func_one` goes from 406 ms to 156 ms, and the
official takes 156 ms.

The `NumGet` and the `NumPut` of a loop are in the compiled path: the type
of the number is the text of the source, and it is resolved when the loop
is compiled; the value, the target and the offset are the values of the
loop, and they are read from the registers of it. Caching the backing buffer
cell across turns in compiled `NumPut`, skipping unused pointer lookups on
statement-level `NumPut`, and decoding via `from_le_bytes` (commit
`8122aa99`) brought `numput_numget` from 141 ms (137 ms on the fine clock)
down to 47 ms (49 ms on the fine clock) against 110-140 ms in the official
(`0,34x`–`0,43x`).

The path that the tree does **not** compile is the call of a method whose
arguments read a variable of the loop: the arguments of the call of a
method are read by the call itself, the way the generic path reads them,
and the generic path reads the variables of the script -- the value of a
variable of the loop is not there while the loop runs. A call of a method
whose arguments are the ones that read nothing is compiled.

The property of an object and the method of an array are in the compiled
path: measured on the machine of the owner, the workload `obj_prop`
answers in 250 ms where the official takes 375 ms, and the workload
`arr_push_get` answers in 94 ms where the official takes 172 ms.

## Where the time goes

### The statement that is not compiled

The tree interprets the tree of the script at the run: the official
compiles it. A statement that the fast loop does not cover pays for the
machinery of the statement (the step of the report, the line, the name of
the operation, the clock of the timeout) and for the evaluation of its
value.

Measured by bisection (`s := "abc"`, five million turns):

| The part | The cost |
|---|---:|
| the literal: the allocation of the text and the search of the `%` | ~54 ns |
| the statement: the machinery and the write of the variable | ~90 ns |
| the call: the lookups (the functions of the script, the types of the classes, the walk of the scopes, the built-in variables) | ~59 ns |
| the call: the lookup of the built-in, the arguments and the handler | ~150 ns |

### The text owns its data

`Value::Text(String)` owns the text: every evaluation of a literal and
every write of a variable copies it. The official shares it -- the value
of a text is a pointer to the text of the script -- and its copy is free.

The refactoring that would match it is `Value::Text(Rc<str>)`. It touches
every use of the value in the tree, and it is the biggest of the three
levers.

### The text of the arguments is copied

The workload of the strings (`strings_500k`) is 406 ms where the official is
63. The calls are compiled now, so the rest is the work of the built-ins
themselves -- and the measurement says that the work of the search is a
small part of it (the focused workload of the search): the
`InStr` of a haystack of 55 characters costs 126 ns in the tree and 62 ns
in the official, and a search that walks the haystack eight bytes at a
time (instead of one byte at a time) does not move the number.

What is left is the **copy of the text of the arguments**: the haystack of
the call is a variable, and reading it copies the string; the needle of
the call is a literal, and reading it makes a string. The official copies
neither: the value of a text is a pointer to the text of the script, and
the copy is a count of references.

⇒ The lever is `Value::Text(Rc<str>)`. It touches every use of the value
in the tree, and it is the one that answers the texts, the arguments and
the writes of a variable. The compiler points at **979** of them, and
897 of the 979 are the same shape -- the construction of a text -- so
the change is mechanical for them and by hand for the rest.

### The properties are a tree of names

The properties of an object are a `BTreeMap<String, Property>`: the
lookup of a property is a walk of the tree with a comparison of names.
The official compiles the property to a slot of the object.

The fast path of the tree (`Runtime::read_property_fully`) answers a
property of an object that has no accessor without walking the classes,
but it pays for two lookups: the one of `__class` (a property that would
change the meaning of the name) and the one of the property itself.
Removing the two (an experiment, not a change) is 9% of the workload of
the properties.

A table of hash for the scope of a call was tried and measured: it is
neutral (the scope of a call holds two or three names, and a tree of that
size is as good as a hash).

The property in the compiled path (the fast loop compiles the loops of
the integers, of the registers and of the calls of the built-ins) was
tried and measured too: it is neutral as well (1875 ms before, 1891 ms
after, over five million turns of `o.a := o.a + o.b`). The work of the
property is the lookup of the name, and the machinery of the statement is
a small part of it.

### The call of a method builds a scope

The workload of the method (`c.Inc(1)`, 300 thousand turns) is 218 ms in
the tree and 109 ms in the official, and the measurement says that the
call itself is the cost: a method that writes nothing costs 156 ms in the
tree and 110 ms in the official (the focused workload
of the method).

The call makes a scope and the scope is a `BTreeMap<String, Value>`: the
table of it is made, the name of `this` is copied into it, the name of
every parameter is copied into it, the class of the method is copied in
lower case, and the frame of the call carries the name of the method.

**The name of the method is built once** (the commit `62a5aa53`): it was
built in every call -- the lookup of the class, the copy of its name and
the text of the name -- and it is the same in every call, because the
method that owns it is the one that is called. The call of the method
went from 250 ms to 203 ms, and the method that writes nothing went from
203 ms to 156 ms.

The rest of the call is the scope: the names of `this` and of the
parameters are copied into it in every call, and the name of a variable
that the official compiles is a name that the tree copies.

**The names of the scope and of the frame are shared** (the commit
`d6c22875`): every call of a function or of a method allocated a fresh
`BTreeMap<String, Value>`, copied `this` and every parameter name into
it, and the frame of the call carried the name of the method. The names
are shared now -- they are the same text in every call of the same
method -- and the scopes of the calls are pooled.

### The phases of a call

The workload `call_phases` measures the phases of a call, each one with
the same number of turns, so that the difference between two phases is
the difference between the two works:

| The phase | The tree | The official v2.0.32 |
|---|---:|---:|
| the loop alone | 16 ms | 16 ms |
| the call of a method with one parameter | 297 ms | 187 ms |
| the call of a method with two parameters | 375 ms | 203 ms |
| the call of a function of the script | 234 ms | 157 ms |

The loop alone is the floor, and the tree pays the same as the official
there. The call of a method with one parameter costs the tree 110 ns
more than the official, and the call of a function costs it 77 ns more.

The second parameter of the method costs the tree 78 ns and the official
16 ns: the body of the method is interpreted, and the interpretation of
the `n + m` is the difference. That is the same gap that the statement
that is not compiled is.

The name of the method is built once (the commit `62a5aa53`) and the
names of the scope and of the frame are shared (the commit
`d6c22875`), and the resolution is still two lookups of a tree of names
in every call.

The clock of the statement was measured too: the tree reads the clock of
the machine in every call, for the diagnostic of the slow steps, and
taking that read out is neutral -- 297 ms before, 297 ms after. The cost
of the call is not the clock.

### The lookup of the method is not the handicap

The workload `method_lookup` calls a method of a class with one method
and a method of a class with nineteen, and the difference between the
two phases is the cost of the lookup of the method -- the tree of names
of the class and the name of the method.

| The phase | The tree | The official v2.0.32 |
|---|---:|---:|
| the call of the only method of a class | 313 ms | 172 ms |
| the call of the last method of a class with nineteen | 328 ms | 188 ms |

The lookup costs the tree fifteen nanoseconds more than it costs the
official -- fifteen against sixteen, in fact -- and the tree is a
hundred and forty nanoseconds behind in the call. **The lookup is not
where the tree loses**, and resolving the method once per access site
would buy nothing.

What is left is the body of the method and the arguments of the call:
the tree interprets both, and the official compiles both. The compiled
path of the tree has no `Call`, and that is the whole of the gap.

### The scope of the call is the interpreter

The workload `method_scope` calls three methods of the same class, whose
bodies are the same -- the method returns its first parameter and does
not read the others -- and with one, two and three parameters. The
difference between the phases is the cost of a parameter in the scope of
the call.

| The phase | The tree | The official v2.0.32 |
|---|---:|---:|
| the call of a method with one parameter | 296 ms | 187 ms |
| the call of a method with two parameters | 329 ms | 188 ms |
| the call of a method with three parameters | 343 ms | 188 ms |

The official does not pay for the parameters at all: they are slots of
the compiled frame. The tree pays thirty nanoseconds for each one, and
that is not the lookup of the name -- it is the evaluation of the extra
argument and the writing of it in the scope, which the interpreter does
and the compiler does not.

So the handicap of the tree in a call is one thing and not three: **the
tree interprets the body of the method and the arguments of the call,
and the official compiles both.** The compiled path of the tree has no
variant for the call of a method of a class -- the `FastExpr` has the
`Builtin`, the property and the index, and the `FastStmt` has the `Push`
of an array, but neither of them has the call of a method -- and that is
why a loop whose body calls a method of a class is not compiled at all.

### The loop over a collection is compiled

The loop of the v2 over an array, over a map or over an object was
interpreted: the tree walked the collection and asked the runtime for
the value of every element, one statement at a time.

The loop is compiled now (the commit `dd156fab`): the collection is
walked once, the built-ins of the body are resolved when the loop is
compiled, and the caches of the loop -- the `NumGet`, the `Buffer.Ptr`,
the plain objects, the `Array` and the `Map` -- live in the loop and are
dropped when it ends.

Measured on the machine of the owner (the median of seven runs, the
workloads of the suite):

| The workload | Before | After | The official |
|---|---:|---:|---:|
| `arr_push_get_for_500k` | 94 / 203 / 125 | 16 / 16 / 15 | 31 / 47 / 46 |
| `map_set_get_for_200k` | 94 / 93 / 47 | 16 / 16 / 0 | 16 / 31 / 15 |
| `obj_prop_500k` | 31 | 16 | 31 |
| `noop_300k` | 78 | 63 | 47 |
| `while_2M` | 125 | 110 | 188 |
| `func_call_500k` | 140 | 125 | 79 |
| **The total (without the fib)** | **1625** | **1018** | **1155** |

The total of the suite is below the official's for the first time, and
the map of the loop is three times faster than it was.

Two of the workloads are slower: the call of a method (79 against 93) and
the `NumPut`/`StrGet` of a loop (63 against 78). They are the price of
the caches of the loop, and they are the next thing to look at.

The loop of the v1 is not compiled: the v1 has a loop of its own, and the
tree reads it the way the v1 does.

### The types of the classes were a walk of names

The question "is this name a class of the runtime?" was a walk of the
tables of the classes -- twenty-six names of the types, fourteen of the
errors and the nested ones of the windows -- in **every** call of a
built-in. The names are in a table with the hash of the tree now
(`crates/ahk/src/fast_hash.rs`), and the call of a built-in is 8,4%
cheaper (the commit `da809998`).

## The next steps, by the size of the gain

The three big levers are done: the text of a value is shared (the commit
`7ed78a64`), the names of the scope and of the frame are shared (the
commit `d6c22875`), and the path that is compiled carries the
properties, the methods of an array and the loop over a collection (the
commits `7763f5d5` and `dd156fab`).

What is left:

1. **The three workloads that the tree is slower in.** The call of a
   function (1,6x), the call of a method (1,8x) and the `NumPut`/`NumGet`
   of a loop (1,7x) are slower than the official, and
   `scripts/perf-compare.py` is what measures them. The call of a method
   is the one that the caches of the loop were blamed for, and the
   blame is wrong: the compiled path has no variant for the call of a
   method of a class (the `FastExpr` has the `Builtin`, the property and
   the index, and the `FastStmt` has the `Push` of an array, but neither
   of them has the call of a method), so a loop whose body calls a method
   of a class is not compiled at all -- the call goes through the
   interpreter and the scope of it is the cost.
2. **The properties that are slots.** The property of an object is
   resolved once per access site and not in every access.
3. **The loop of the v1.** The loop of the v1 is not compiled: the v1
   has a loop of its own, and the tree reads it the way the v1 does.

## A deviation that the plan already carries

The tree interpolates `%name%` in a text of the v2, and the official v2
does not: measured on the binary of the v2.0.32, `"a%b%"` is the text
itself, and the source of the upstream has no `case '%'` at all (the v1
has it, at `source/script.cpp:1985`). The plan of the v1 says it: *"The
tree today interpolates `%nome%` with the variable of the script -- it is
the rule of the v1 with the sense of the v2, and the plan corrects it per
dialect"* (`docs/ahk-v1-dialect.md`).

The correction is a change of the tests of the tree -- about a hundred of
them use the substitution in the v2 -- and of the sense of the rule of the
v1, where `%name%` is the variable of the *environment* and the `#NoEnv`
turns the substitution off.
