# `WinGetActiveStats`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · GUI windows and window automation

## Description

Reads the active window's title/pos.

## Syntax

```ahk
WinGetActiveStats &T, &X, &Y, &W, &H
```

## Parameters

5..255.

## Return value

Nothing.

## Example

```ahk
WinGetActiveStats &t, &x, &y, &w, &h
```

## Related

[`WinActivate`](WinActivate.md), [`WinExist`](WinExist.md)
