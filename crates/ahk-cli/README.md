# ahk-cli

The runner of the tree: a script on standard input, a report on standard
output, in JSON.

```sh
echo 'MsgBox "hello"' | ahk
```

The script may come from a file instead: the path of the file is the one
argument that is not a flag.

```sh
ahk script.ahk
```

The file is read instead of the standard input, the includes of the script
are resolved against the directory of the file, and the runtime knows the
file, so that `A_ScriptFullPath` is a full path and the `Edit` and the
`Reload` of v2.0.31 have a file to work with. A relative path is joined
with the working directory.

The exit code is 0 when the script finished, 1 when it failed, and 2 when it did
not parse; the report is on standard output in the three cases.

```json
{"ok":true,"steps":[{"line":1,"op":"MsgBox","detail":"hello","ok":true,"elapsed_ms":0}],"output":["hello"],...}
```

## The dialects

The default dialect is **AutoHotkey v2.0.31**. `--dialect=v1` asks for
**v1.1.37.02**, which the same binary speaks.

## The flags

| Flag | What it does |
| --- | --- |
| `--dialect=v1` | Speak the v1 dialect instead of the v2 |
| `--timeout=N` | Stop the script after N seconds (60 by default) |
| `--surface` | Print the names of the dialect and the state of each, and read no script |
| `--surface --json` | The same table, as JSON |
| `--surface --dialect=v1` | The table of the v1 |
| `--help` | Print the usage, and read no script |

The one argument that is not a flag is the path of the file of the
script, and it is optional: without it the script comes from the standard
input, and the runtime has no file.

## The report

`ok`, `steps`, `output`, `warnings`, `notes`, `error`, `error_line`,
`exit_code`, `elapsed_ms`, `variables`. A statement that completes produces a
step; a statement that fails stops the script and is reported once, in `error`
and `error_line`, and it is not a step.

## Documentation

[`docs/runner-and-report.md`](https://github.com/danalec/ahk/blob/master/docs/runner-and-report.md)
is the guide: the command, the flags, the report field by field, the guard rails
and the emergency stop.

## Licence

Apache-2.0.