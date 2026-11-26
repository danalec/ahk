# `A_ScriptHwnd`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · variable · Der Parser und die Ausführung des Skripts

## Beschreibung

The handle of the hidden window of the script: the window that owns the messages of the session. The number changes with the run -- the v2.0.31 answered 6557770 in one run and 59116662 in another one.

## Syntax

```ahk
A_ScriptHwnd 
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
