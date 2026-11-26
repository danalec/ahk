# `ScriptHwnd`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · variable · GUI-Fenster und Fensterautomatisierung

## Beschreibung

The handle of the hidden window of the script, the name the v2 gives it. The window is the one that owns the messages of the session, and the number changes with the run.

## Syntax

```ahk
ScriptHwnd 
```

## Parameter

nothing; it is a variable, not a call

## Rückgabewert

the handle of the window, as a number

## Fehler

the read fails with the reason of the desktop when the desktop has no window of the script; the write is refused.

## Beispiel

```ahk
FileAppend A_ScriptHwnd, "*"
```
