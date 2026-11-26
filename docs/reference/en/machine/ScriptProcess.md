# `ScriptProcess`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*KeySharp* · class · The machine, the processes and the environment

## Description

The class of a running script process (ks): HasExited/ExitCode/ExitTime/Kill/StdOut/StdErr/StdIn.

## Syntax

```ahk
P := RunScript(...)
```

## Parameters

?.

## Return value

A ScriptProcess object.

## Example

```ahk
p := RunScript("c.ahk")
```

## Related

[`RunScript`](RunScript.md), [`HasExited`](HasExited.md), [`Kill`](Kill.md)
