# `A_DD`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · variable · Datei- und Verzeichnisoperationen

## Beschreibung

The day of the month of the local time, in two digits.

## Syntax

```ahk
A_DD 
```

## Parameter

nothing; it is a variable, not a call

## Rückgabewert

the day, as a text of two digits

## Fehler

the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

## Beispiel

```ahk
FileAppend A_DD, "*"
```
