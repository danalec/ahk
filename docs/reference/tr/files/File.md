# `File`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · method · `Error` · Dosya ve dizin işlemleri

## Açıklama

The class of an open file of the machine. The class is not made: `File(...)` answers "Invalid base."; the file is made by the `FileOpen` built-in, and it is the only way to have one.

## Sözdizimi

```ahk
File 
```

## Parametreler

nothing that makes it; the arguments of `FileOpen` open the file

## Dönüş değeri

the object of the class `File`

## Hatalar

the call of the class is refused with "Invalid base.".

## Örnek

```ahk
f := FileOpen("notes.txt", "w")
f.Close()
```
