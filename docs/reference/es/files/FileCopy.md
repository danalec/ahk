# `FileCopy`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Operaciones de archivos y directorios

## Descripción

Copies a file, or every file that a pattern names. A destination that ends with a separator, or that is a folder, is the folder the files are copied into.

## Sintaxis

```ahk
FileCopy Source, Dest, Overwrite
```

## Parámetros

- **Source** — Path/wildcard.
- **Dest** — Destination.
- **Overwrite** — 1 to overwrite.

## Valor de retorno

nothing

## Errores

Error `Failed` when the source is not there, or when the destination is there and the overwrite is not asked for.

## Ejemplo

```ahk
FileCopy "a.txt", "b.txt"
```

## Relacionados

[`FileMove`](FileMove.md), [`DirCopy`](DirCopy.md)
