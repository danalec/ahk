# `WinClose`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · GUI-Fenster und Fensterautomatisierung

## Beschreibung

Closes a window. It closes the window of the script itself: the listing of the compositor publishes no close request for a window of another program.

## Syntax

```ahk
WinClose WinTitle, WinText, SecondsToWait
```

## Parameter

- **WinTitle** — The criteria.
- **SecondsToWait** — Then force.

## Rückgabewert

nothing

## Fehler

a window that is not there answers 'Target window not found.'; a window of another program cannot be closed this way.

## Beispiel

```ahk
WinClose "Untitled"
```

## Verwandt

[`WinKill`](WinKill.md), [`WinActivate`](WinActivate.md)
