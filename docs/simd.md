# What SIMD improves, and what it does not

The tree has **no hand-written SIMD intrinsics**. Every gain from a SIMD
build comes from two places: the LLVM auto-vectorizer turning a tight
loop into vector instructions, and the standard library's own vectorized
primitives (the byte search behind `str::find`, `memcpy`/`memmove`, the
UTF conversions). A SIMD build turns those on by raising the CPU
baseline; it does not add a new code path.

That is what the builds do:

| Build | The flags | Where |
|---|---|---|
| CI `simd` job | `x86-64-v2` / `v3` / `v4`, `neoverse-n1`, `apple-m1` | `.github/workflows/ci.yml` |
| AUR `ahk-git` | `-C target-cpu=native` (built on the user's machine) | `packaging/aur-git/PKGBUILD` |
| Nix `ahk-simd` | `x86-64-v3` / `neoverse-n1` / `apple-m1` (reproducible) | `flake.nix` |

The plain `ahk` package stays the portable baseline (`x86-64`, SSE2 only);
the SIMD builds are opt-in and trade portability for speed. `v2` adds
SSE4.2/popcnt, `v3` adds AVX2/FMA/BMI2, `v4` adds AVX-512; on ARM the NEON
baseline is already on and the named CPUs add the wider extensions.

The sections below name the functions whose hot path is a loop the
vectorizer can widen, with the file that holds it. The gain is a shape,
not a benchmark: it depends on the data (ASCII text vectorizes; a string
of astral characters does not, because a `char` is four bytes and the
loop is over code points, not bytes).

- [Text search and replace](#text-search-and-replace)
- [Text scanning and case](#text-scanning-and-case)
- [The encodings](#the-encodings)
- [Collections and memory moves](#collections-and-memory-moves)
- [The screen](#the-screen)
- [What SIMD does not improve](#what-simd-does-not-improve)

---

## Text search and replace

`InStr`, `StrReplace`, `StrSplit` (and the `find` of the `ks` extensions)
all go through `find_in_sense` (`crates/ahk/src/builtins.rs`). In the
case-sensitive sense it is `text.find(needle)` — the standard library's
two-way substring search, whose inner byte comparison is the vectorized
`memchr`-style scan of `core`. On a SIMD baseline that scan reads 16/32
bytes at a time instead of one, so a long haystack with a rare needle is
where the gain shows.

The case-insensitive senses go through `find_ignoring_case`, a fold-and-
compare loop over the text. It is a loop the vectorizer can widen for
ASCII runs, but the fold branches on the case of each character, so the
gain is smaller than the sensitive path.

`StrReplace` repeats the search over the rest of the text once per
replacement, so it inherits the search's gain multiplied by the number
of hits.

## Text scanning and case

`StrLen`, `Trim`, `LTrim`/`RTrim`, `StrLower`/`StrUpper` and the split of
`StrSplit` are scans over the characters or the bytes of a string
(`crates/ahk/src/builtins.rs`). The whitespace test of `Trim` and the
ASCII case fold are byte predicates the vectorizer widens; the counting
of `StrLen` over a UTF-8 string is a decode loop that widens on ASCII
runs (one byte per character) and falls back to scalar on multi-byte
sequences.

## The encodings

The UTF-8 ↔ UTF-16 conversions (`crates/ahk/src/text16.rs`, and the
encode/decode of the file layer in `crates/ahk/src/files.rs`) are loops
over code units. The ASCII fast path — a run of units below 0x80 — is a
copy-with-widen/narrow that the vectorizer turns into a pack/unpack
instruction, so reading and writing a mostly-ASCII file is where a SIMD
build pays. The surrogate handling of `text16.rs` (the private-use area
that stands for a lone surrogate) is a branch per unit and stays scalar.

## Collections and memory moves

`Array.Clone`, the slice of `Array`, `Join`, and the growth of the
`Array`/`Map` tables move `Value` vectors with `memcpy`/`memmove`, which
are SIMD on every baseline worth naming. `Sort`
(`crates/ahk/src/builtins.rs`) is a comparison sort: the comparisons
themselves are scalar, but the moves of the elements and the extraction
of the sort key (a text or a number per item) use the vectorized moves
and the text scans above. The `Map` is a `BTreeMap`, so its lookups are
comparison trees, not a SIMD hash.

## The screen

`PixelSearch` and `PixelGetColor` read the capture buffer and compare
pixels (`crates/ahk/src/pictures.rs` and the screen of the desktop). A
search over a region is a per-pixel compare against a target colour with
a variation — a loop over the buffer the vectorizer widens, so a large
region with a tight variation is the screen's SIMD case. The capture
itself is the machine's (`grim`, `XGetImage`, `BitBlt`), not the tree's,
and is not changed by the flags.

## What SIMD does not improve

Honesty about the rest:

- **The interpreter.** The dispatch of a statement, the parse, the
  evaluation of an expression are branchy tree-walks, not data-parallel
  loops. A SIMD build does not make a script of many small statements
  faster.
- **The scalar math.** `Abs`, `Floor`, `Ceil`, `Round`, `Mod`, `Sqrt`,
  `Sin`, `Cos`, `Log`, `Exp` (`crates/ahk/src/math.rs`) act on one number
  at a time; the transcendental ones are libm calls. There is no array
  math to vectorize, so the gain is nil unless a script feeds them in a
  tight loop the vectorizer can see.
- **The `Map` lookups.** A `BTreeMap` is a comparison tree; no hash, no
  SIMD.
- **The number → text of `Format`.** The digit extraction of
  `format_integer`/`format_float` (`crates/ahk/src/builtins.rs`,
  `crates/ahk/src/float_text.rs`) is a short scalar loop per number; the
  width of a single number is too small to vectorize.
- **Anything astral-heavy.** A loop over `char` is a loop over four-byte
  code points; the vectorizer's byte tricks apply to the ASCII fast path,
  not to a string of emoji.

So the SIMD build is a win for scripts that move a lot of bytes — long
text searched or replaced, mostly-ASCII files read or written, big
arrays copied or sorted, large pixel regions — and a no-op for scripts
that are mostly control flow and scalar arithmetic. The CI `simd` job
builds and tests the baselines so a regression in any of these paths is
caught; it does not claim a speedup the loop cannot give.
