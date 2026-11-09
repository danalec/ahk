# t2 — top 10 verified facts about the Rust refusals

Every fact below was measured, not inferred. The measurement commands and the
source lines are given with each one. The evidence table is
[`rust-refusals.tsv`](rust-refusals.tsv); the long form is
[`rust-refusal-analysis.md`](rust-refusal-analysis.md).

## The corrected headline numbers

Measured with `.\target\release\ahk.exe --surface` on 2026-11-09:

```
the surface of v2 has 833 entries: 643 answer with a value, 190 are refused, 0 are pending.
```

`docs/ahk-x11-comparison.md` says 725 / 506 / 219 at `:772`, and 93 real
handlers / 253 `not_carried` at `:355-356`. Both are stale. The built-in table
is **346 rows** (346 matches of `^    Builtin {` in `crates/ahk/src/builtins.rs`),
of which **181** are `handler: not_carried` and **165** real — the doc's 346
holds, and so do its 242 commands / 103 functions / 131 variables (`:808`); the
surface counts and the handler counts do not.

---

## 1. 169 of the 190 refusals are one sentence, and it is a lie for 115 of them

`this is a name of Windows, and this desktop is not Windows.` covers 169 rows
(`builtins.rs:3781`, variant 0, plus `variables.rs:90`). Judged name by name
against what Linux/Wayland actually publishes: **60 are genuinely Windows-only,
15 are absent by the runtime's own design, and 115 are implementable** — 31 of
those "yes, trivially". Total: 31 yes / 74 likely / 85 no.

## 2. The cheapest cluster is the processes: 8 names, one trait, `/proc`

`ProcessExist`, `ProcessGetName`, `ProcessGetParent`, `ProcessGetPath`,
`ProcessClose`, `ProcessSetPriority`, `ProcessWait`, `ProcessWaitClose` all
refuse with the Windows sentence, but `kill(pid,0)`, `/proc/<pid>/comm|stat|exe`
and `renice(2)` answer every one of them. The only thing missing is the method:
`crates/ahk/src/desktop.rs:237-282` — the `Processes` trait has `run`, `launch`,
`run_and_wait`, `env_get`, `env_set` and nothing else. `RunWait` already works
(verified: it is `value` on the surface), so the process machinery exists.

## 3. Three lock-key commands are a `press_key` each

`SetCapsLockState`, `SetNumLockState`, `SetScrollLockState` refuse as Windows,
but the uinput keyboard of the box can press the keys: `crates/ahk/src/keys.rs:171`
resolves `capslock`, and `keys.rs:770-772` pins the codes
(`CapsLock` 58, `NumLock` 69, `ScrollLock` 70). The capability is already
driven by `Send`.

## 4. `OnClipboardChange` is refused while the clipboard works

The clipboard capability is implemented — `crates/ahk-box/src/desktop.rs:367-446`
(`clipboard_read`, `clipboard_write`, `wl-paste`/`wl-copy`) — and `A_Clipboard`
and `ClipWait` are `value` on the surface. `OnClipboardChange` alone is refused
as Windows. `wl-paste --watch` is the missing piece, not a platform limit.

## 5. `StrGet`/`StrPut`/`StrPtr` are refused next to `NumGet`/`NumPut`, which work

Verified: `b := Buffer(64); NumPut("Int",42,b); NumGet(b,"Int")` answers `42`,
and the string encoder for native calls is already in the tree
(`crates/ahk-box/src/native.rs:731` uses `ahk::dll::encode` with
`NativeType::AStr`). The three `Str*` names refuse as Windows; the only thing
missing is the pointer-shaped wrapper over machinery that is already there.

## 6. The windows gap is smaller than the refusal list says

`WinExist`/`WinActive`/`WinGetTitle` work through the two protocols the client
already binds (`crates/ahk-box/src/wayland/linux.rs:122,142`). Those protocols
carry more than the tree uses: a stable `identifier` and a `close` request
(`ext-foreign-toplevel-list-v1`), a `geometry` rectangle
(`protocols/cosmic-toplevel-info-unstable-v1.xml:229-243`), and an
`activated`/`fullscreen` state (`:215-217`). That makes `WinGetID`, `WinGetList`,
`WinGetCount`, `WinGetClass`, `WinClose`, the four `WinWait*` and the four
`Group*` implementable now; `WinGetPos`, `WinGetClientPos` and `WinGetMinMax`
follow from the geometry/state events the same client already receives.
`WinMove`/`WinSetTitle`/`WinHide`/`WinShow` are honestly impossible: Wayland
gives placement and mapping to the compositor.

## 7. The GUI gap is invisible in the surface and is the largest in the tree

**148** method entries — 59 `Gui.*`, 29 `Gui.Control.*`, 19 `Gui.Menu.*`,
15 `Gui.ListView.*`, 15 `Gui.TreeView.*`, 11 `Gui.StatusBar.*` — are reported as
`value`. They are not: `crates/ahk/src/classes/gui.rs:215-258` refuses the whole
family, and the probe agrees — `g := Gui()` succeeds, `g.Show()` fails with
"Gui cannot show the window: a window is a Windows window, and this desktop is a
Wayland session without one." A Wayland client can make windows, so this is a
missing implementation, not a refusal. **The headline "643 answer with a value"
overstates the tree by about 148 names.**

## 8. Zero stale refusals — and it is provable by counting

181 refused commands+functions (156 commands + 25 functions) = 181
`handler: not_carried` rows = 181 refusal variants. The surface derives the
state from the variant alone (`crates/ahk/src/surface.rs:273-276` →
`builtins.rs:3768-3773`), so a row with a real handler but a refusal variant
would break the equality. It does not break. The 9 refused variables are the
same: `variables.rs:90-142` owns exactly those reasons.

Two smaller inconsistencies do exist: `ObjAddRef` is `value` while `ObjRelease`
is refused (and `ObjAddRef()` on an object fails with "Parameter #1 of
ObjAddRef is invalid."), and the table carries one non-v2 name, `Screenshot`
(`builtins.rs:665`), which is why 346 rows cover 345 v2 names.

## 9. `Desktop` is fifteen capability traits; the doc says ten

`crates/ahk/src/desktop.rs:920-936` names
`Keyboard + Pointer + Screen + Windows + Processes + Files + Clipboard + Timers
+ Hotkeys + Native + Sound + Machine + Dialogs + Network + Pictures` — fifteen.
`docs/ahk-x11-comparison.md:343-346` lists ten, missing `Sound`, `Machine`,
`Dialogs`, `Network`, `Pictures`. `AGENTS.md`'s "fifteen" is right. The same
paragraph's "every implementor writes the ten" is wrong twice: the traits have
refusing defaults (`desktop.rs:46-58`), and `X11Desktop` relies on them in ten
empty impls (`ahk-box/src/x11.rs:1389-1398`).

## 10. The report has 10 keys, and the docs list 9

`'' | .\target\release\ahk.exe` answers:

```json
{"ok":true,"steps":[],"output":[],"warnings":[],"notes":["no uid on this platform; the session environment is unknown"],"error":null,"error_line":null,"exit_code":null,"elapsed_ms":0,"variables":{}}
```

`warnings` is in the real payload and absent from
`docs/ahk-x11-comparison.md:404-405`. `docs/runner-and-report.md:119-127` has it
right — but `:48`'s "the keys come out in alphabetical order" is also false:
the real stdout is in struct order (`ok` first, `elapsed_ms` near the end).

---

## The refusal table's dead entries, as a closing fact

`refusal_message` (`builtins.rs:3779-3829`) has 22 numbered reasons; the v2
surface uses **11** of them. The doc's "five reasons, variants 0-4" (`:358-366`)
is a photograph of an older table: **only variant 0 is still used**. Variant 1
("no hotkeys yet"), 2 ("does not know the date"), 3 ("one script that ends when
it ends") and 4 ("cannot serve that with the files it has") now have **no user
at all** — hotkeys, dates and files are implemented — and so do 7 more (5, 6,
16, 17, 18, 19, 20: sound, dialogs, network and pictures all landed).
