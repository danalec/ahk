# `WinGetPos`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · GUI windows and window automation

## Description

Where the window is and how big it is, as the compositor reports it.

## Syntax

```ahk
WinGetPos X, Y, W, H, WinTitle
```

## Parameters

- **Output*** — Variables.
- **WinTitle** — The criteria.

## Return value

nothing; the place and the size are written into the variables of the caller

## Errors

the text of the window and the exclusions are not carried yet; a window that is not there answers 'Target window not found.'

## Example

```ahk
WinGetPos "Untitled", , &x, &y, &w, &h
```

## Related

[`WinMove`](WinMove.md), [`WinGetTitle`](WinGetTitle.md)
