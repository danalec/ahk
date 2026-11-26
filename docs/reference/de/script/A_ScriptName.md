# `A_ScriptName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · variable · Der Parser und die Ausführung des Skripts

## Beschreibung

The name of the script. The script of the runner comes on the standard input and has no file, so the name is an asterisk. A script may write the name, and the value written is the one it keeps.

## Syntax

```ahk
A_ScriptName 
```

## Parameter

nothing; it is a variable, not a call

## Rückgabewert

the text "*", or the text that was written

## Fehler

nothing to read; the write takes any value.

## Beispiel

```ahk
FileAppend A_ScriptName, "*"
```
