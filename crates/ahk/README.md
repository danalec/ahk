# ahk

The language of the tree: a lexer, an AST, a parser and an interpreter for the
AutoHotkey dialects, with no I/O of its own.

The dialect is **AutoHotkey v2.0.19**. A second dialect, **v1.1.37.02**, is
chosen with the `--dialect=v1` flag of the runner; the two are served by the
same parser and the same runtime, and every rule that differs between them is a
question the tree asks the dialect before it acts.

```rust
use ahk::{parse, Runtime};

let script = parse("FileAppend \"hello\", \"*\"")?;
let report = Runtime::new(&mut desktop, std::time::Duration::from_secs(10)).run(&script);
assert!(report.ok);
```

What drives the machine is handed in: the crate does no I/O, everything it can
do to a desktop goes through the `Desktop` trait, and a host that wants fewer
rights implements fewer capabilities.

## Without the standard library

The crate has a `std` feature, on by default. With it off the crate is
`#![no_std]` and builds for a target that has no `std` at all; the host then
supplies the clock through the `Clock` trait, and the float functions come from
the crate's own `math` module.

```sh
cargo build -p ahk --no-default-features --target thumbv7em-none-eabihf
```

## Documentation

- [`docs/language-guide.md`](https://github.com/danalec/ahk/blob/master/docs/language-guide.md)
  -- the language, for someone learning it or looking a construct up.
- [`docs/ahk-v2-reference.md`](https://github.com/danalec/ahk/blob/master/docs/ahk-v2-reference.md)
  and [`docs/ahk-v1-reference.md`](https://github.com/danalec/ahk/blob/master/docs/ahk-v1-reference.md)
  -- every name of each dialect, with the state the tree answers for it.

## Licence

Apache-2.0.