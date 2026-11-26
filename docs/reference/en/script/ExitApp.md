# `ExitApp`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · Parser and script execution

## Description

Ends the script with a code. The code goes into the exit_code of the report.

## Syntax

```ahk
ExitApp ExitCode
```

## Parameters

- **ExitCode** — The code the process exits with (default 0).

## Return value

the code

## Errors

none.

## Remarks

OnExit callbacks run before the process ends.

## Example

```ahk
ExitApp 3
```

## Related

[`Exit`](Exit.md), [`Reload`](Reload.md), [`OnExit`](OnExit.md)
