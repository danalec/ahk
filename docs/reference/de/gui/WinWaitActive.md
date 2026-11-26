# `WinWaitActive`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · GUI-Fenster und Fensterautomatisierung

## Beschreibung

Waits for a window whose title matches to have the focus, and answers the identifier of it.

## Syntax

```ahk
WinWaitActive …
```

## Parameter

WinTitle, WinText, [Timeout], [ExcludeTitle], [ExcludeText]

## Rückgabewert

the identifier of the window as text, or 0 when the timeout ran out

## Fehler

a session that is not Wayland answers "this desktop waits for the windows of a Wayland session, and this session is not Wayland"; the two exclusions are refused with "the two exclusions are not carried yet."

## Beispiel

```ahk
WinWaitActive "Calculator", , 5
```
