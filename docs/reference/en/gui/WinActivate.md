# `WinActivate`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · GUI windows and window automation

## Description

Brings a window to the front. It acts on the window of the script itself: the listing of the compositor publishes no way to raise a window of another program.

## Syntax

```ahk
WinActivate WinTitle, WinText, ExcludeTitle, ExcludeText
```

## Parameters

- **WinTitle** — Title/ahk_class/ahk_exe/ahk_id/ahk_pid.
- **WinText** — A text the window must contain.
- **Exclude*** — Criteria to skip.

## Return value

nothing

## Errors

a window that is not there answers 'Target window not found.'; a window of another program cannot be activated this way.

## Remarks

On Wayland the compositor may refuse focus; the tree reports the reason.

## Example

```ahk
WinActivate "Untitled"
```

## Related

[`WinWait`](WinWait.md), [`WinActive`](WinActive.md), [`WinClose`](WinClose.md)
