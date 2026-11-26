# `A_ThisFunc`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · variable · Der Parser und die Ausführung des Skripts

## Beschreibung

The name of the function that runs.

## Syntax

```ahk
A_ThisFunc 
```

## Parameter

nothing; it is a variable, not a call

## Rückgabewert

the name, as a text; the empty text when no function runs

## Fehler

nothing to read; the write is refused.

## Beispiel

```ahk
FileAppend A_ThisFunc, "*"
```
