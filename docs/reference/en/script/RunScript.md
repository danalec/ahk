# `RunScript`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*KeySharp* · function · Parser and script execution

## Description

Starts a script in a new process and answers a ScriptProcess (ks).

## Syntax

```ahk
P := RunScript(Path, Args)
```

## Parameters

- **Path** — The script.
- **Args** — The arguments.

## Return value

A ScriptProcess.

## Example

```ahk
p := RunScript("child.ahk")
```

## Related

[`ScriptProcess`](ScriptProcess.md), [`Run`](Run.md)
