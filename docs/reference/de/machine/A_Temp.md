# `A_Temp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · variable · Die Maschine, die Prozesse und die Umgebung

## Beschreibung

The folder of the temporary files of the machine: `TMPDIR`, and `/tmp` when the variable is not there.

## Syntax

```ahk
A_Temp 
```

## Parameter

nothing; it is a variable, not a call

## Rückgabewert

the path of the folder, as a text

## Fehler

the read fails with "the machine has no such folder." when the machine has no such folder; the write is refused.

## Beispiel

```ahk
FileAppend A_Temp, "*"
```
