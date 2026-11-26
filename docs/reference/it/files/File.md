# `File`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · method · `Error` · Operazioni su file e directory

## Descrizione

The class of an open file of the machine. The class is not made: `File(...)` answers "Invalid base."; the file is made by the `FileOpen` built-in, and it is the only way to have one.

## Sintassi

```ahk
File 
```

## Parametri

nothing that makes it; the arguments of `FileOpen` open the file

## Valore di ritorno

the object of the class `File`

## Errori

the call of the class is refused with "Invalid base.".

## Esempio

```ahk
f := FileOpen("notes.txt", "w")
f.Close()
```
