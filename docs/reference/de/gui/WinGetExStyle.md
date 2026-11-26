# `WinGetExStyle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · command · GUI-Fenster und Fensterautomatisierung

## Beschreibung

The extended style of a window: the `WS_EX_*` bits of Windows, the ones that say how the window deals with the other windows and with the screen. The extended style of the window of the script is 256.

## Syntax

```ahk
WinGetExStyle …
```

## Parameter

WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## Rückgabewert

the extended style, as a number

## Fehler

a window that is not there refuses with 'Target window not found.', and the title that was asked about is in the `Extra`; the text of the window and the two exclusions are not carried yet.

## Beispiel

```ahk
FileAppend WinGetExStyle("ZZZ_NO_WINDOW_ZZZ"), "*"
```
