# `FileInstall`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Operaciones de archivos y directorios

## Descripción

Copies a file the way the installer of a compiled script does. Nothing is compiled here: the file is copied. The third parameter is the flag of the overwrite, and a second copy onto a file that is there is a success when the flag is given.

## Sintaxis

```ahk
FileInstall …
```

## Parámetros

Source, Dest, [Overwrite]

## Valor de retorno

nothing

## Errores

Error `Failed` when the source is not there, or when the destination is there and the overwrite is not asked for.

## Ejemplo

```ahk
FileInstall "data.txt", "data.out.txt"
```
