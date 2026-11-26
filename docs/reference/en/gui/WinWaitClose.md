# `WinWaitClose`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · GUI windows and window automation

## Description

Waits for a window whose title matches to be gone, and answers whether it is.

## Syntax

```ahk
WinWaitClose …
```

## Parameters

WinTitle, WinText, [Timeout], [ExcludeTitle], [ExcludeText]

## Return value

1 when the window is gone, 0 when it is still there at the timeout

## Errors

a session that is not Wayland answers "this desktop waits for the windows of a Wayland session, and this session is not Wayland"; the two exclusions are refused with "the two exclusions are not carried yet."

## Example

```ahk
WinWaitClose "Calculator", , 5
```
