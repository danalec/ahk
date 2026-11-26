# `File`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · method · `Error` · Operaciones de archivos y directorios

## Descripción

The class of an open file of the machine. The class is not made: `File(...)` answers "Invalid base."; the file is made by the `FileOpen` built-in, and it is the only way to have one.

## Sintaxis

```ahk
File 
```

## Parámetros

nothing that makes it; the arguments of `FileOpen` open the file

## Valor de retorno

the object of the class `File`

## Errores

the call of the class is refused with "Invalid base.".

## Ejemplo

```ahk
f := FileOpen("notes.txt", "w")
f.Close()
```
