# `FileSetAttrib`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Operaciones de archivos y directorios

## Descripción

Changes the attributes of a file. The attributes come first and the name of the file second, the way the binary of v2.0.31 takes them. The name of the file may be a pattern, and the attributes are changed in every file that the pattern names.

## Sintaxis

```ahk
FileSetAttrib …
```

## Parámetros

Attributes, FileName, [Recurse]

## Valor de retorno

nothing

## Errores

ValueError when the attributes are not one of v2; OSError when the attributes of the file cannot be changed. The third parameter is the recursion: the "R" walks into the subfolders, and any other letter is refused.

## Ejemplo

```ahk
FileSetAttrib "+H", "*.txt"
```
