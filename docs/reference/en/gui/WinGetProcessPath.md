# `WinGetProcessPath`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · command · GUI windows and window automation

## Description

The executable path of a window's process.

## Syntax

```ahk
S := WinGetProcessPath(WinTitle)
```

## Parameters

0..4.

## Return value

The path.

## Example

```ahk
WinGetProcessPath("A")
```

## Related

[`WinActivate`](WinActivate.md), [`WinExist`](WinExist.md)
