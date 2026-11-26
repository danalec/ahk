# `A_Index`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · variable · Der Parser und die Ausführung des Skripts

## Beschreibung

The number of the turn of the loop that runs. It is zero before the first loop, and every loop saves the value it had and gives it back when it ends.

## Syntax

```ahk
A_Index 
```

## Parameter

nothing; it is a variable, not a call

## Rückgabewert

the number of the turn, as a number

## Fehler

nothing to read; a script may write the name, and the loop goes on from the value that was written.

## Beispiel

```ahk
for item in [1, 2, 3]
    FileAppend A_Index, "*"
```
