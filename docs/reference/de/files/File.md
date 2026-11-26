# `File`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · method · `Error` · Datei- und Verzeichnisoperationen

## Beschreibung

The class of an open file of the machine. The class is not made: `File(...)` answers "Invalid base."; the file is made by the `FileOpen` built-in, and it is the only way to have one.

## Syntax

```ahk
File 
```

## Parameter

nothing that makes it; the arguments of `FileOpen` open the file

## Rückgabewert

the object of the class `File`

## Fehler

the call of the class is refused with "Invalid base.".

## Beispiel

```ahk
f := FileOpen("notes.txt", "w")
f.Close()
```
