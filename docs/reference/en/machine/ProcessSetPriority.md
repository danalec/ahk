# `ProcessSetPriority`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · command · The machine, the processes and the environment

## Description

Sets the priority of a process.

## Syntax

```ahk
ProcessSetPriority Priority, NameOrPID
```

## Parameters

- **Priority** — L/B/N/A/H/R or a number.
- **NameOrPID** — The target.

## Return value

Nothing.

## Example

```ahk
ProcessSetPriority "H", pid
```

## Related

[`Process`](Process.md)
