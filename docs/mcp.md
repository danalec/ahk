# The MCP server (`ahk --mcp`)

A Model Context Protocol server for debugging AHK development, over JSON-RPC
2.0 on stdio (one JSON object per line) — the same framing as the rgx MCP. It
exposes the tree's own answers so a client (an editor, an agent) can read the
errors of a script without a shell.

The request is read with the djson on-demand parser (`rgx-core`); the answer is
written with the tree's own JSON writer, so a client reads the same report the
runner writes.

The tools are `check`, `run` and `surface`, and the two that compare the tree
with the binary of the AutoHotkey v1: `v1` and `diff`.

## Run it

```sh
ahk --mcp
```

Configure it in an MCP client as a stdio server whose command is `ahk --mcp`.

## Tools

| Tool | Arguments | Answer |
|---|---|---|
| `check` | `source`, `dialect?` | parse the script: `{ok:true}` or `{ok:false, error, line, column}` |
| `run` | `source`, `dialect?` | the whole report: `output`, `steps`, `error`, `warnings` |
| `surface` | `dialect?` | the count of names and the summary of the dialect |
| `v1` | `source` | what the binary of the v1 said: `output`, `errors`, `code`, `hung` |
| `diff` | `source`, `dialect?` | the answer of the tree and the answer of the v1, side by side |

`dialect` is `v1`, `v2`, `v2.1`, `ks` or `ks20`. When it is left out, the
dialect is the one the script names in its `#Requires` -- the `v1.1` is the
v1, the `v2.0` is the v2 and the `v2.1` is the v2.1 -- and the v2 of the
tree when the script names none.

The includes of the script are resolved before the parse, the way the
runner resolves them: a script that asks for a library that is not there
does not load, and the answer says so with the words of the official and
the line of the directive. The MCP has no path of the script, and the
folder of the script is the working directory of the server.

## Example

A `tools/call` of `check` on a script that does not load:

```json
{"jsonrpc":"2.0","id":1,"method":"tools/call",
 "params":{"name":"check","arguments":{"source":"BoxEx(_this, params*)"}}}
```

answers with the load error, the line and the column:

```json
{"ok":false,"error":"Missing space or operator before this.\n     Specifically: (params*)","line":75,"column":38}
```

So a client can point at the exact place a script fails, the way the runner's
report does, and iterate on the fix without running a shell.

## The reference of the v1

The tree answers a script one way and the AutoHotkey v1 another, and the
difference is the bug to find. The tools `v1` and `diff` run the same script
under the two and give the two answers side by side, so a client finds the
difference without a shell and without the window of the v1.

The binary of the v1 is the one of the install of the v1.1.37.02:

```text
C:\Program Files\AutoHotkey\v1.1.37.02\AutoHotkeyU64.exe
```

The path comes from the variable `AHK_V1` when it is there, and this is the
place of the install when it is not. The script is written in a temporary
file with the directive `#ErrorStdOut` in front of it, so the errors of the
v1 go to the output of the process and not to a dialog of the machine that
waits for a mouse that nobody moves. The run has ten seconds; a v1 that is
still running when the time is over is killed, and the answer says `hung`.

The `v1` tool answers with the whole output, the errors alone (the lines
that carry the `==>` of the v1, with the line of the script in them), the
code of the end and whether the v1 hung. The `diff` tool answers with the
error of the tree and the error of the v1, side by side:

```json
{"tree":{"error":"This global variable has not been assigned a value.","line":1},
 "v1":{"error":"probe.ahk (1) : ==> Call to nonexistent function.","code":2,"hung":false}}
```

A client that reports the two answers one under the other shows the
difference that a client of the tree alone cannot see.

## Methods

`initialize`, `tools/list`, `tools/call` and `ping`. A notification (no `id`)
gets no answer; a method nobody implements gets a JSON-RPC error. The server
keeps no state between requests.
