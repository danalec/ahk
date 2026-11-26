# `DirCreate`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · command · Operaciones de archivos y directorios

## Descripción

Makes a directory, with the folders that are missing in the way.

## Sintaxis

```ahk
DirCreate Path
```

## Parámetros

- **Path** — The directory.

## Valor de retorno

nothing

## Errores

OSError when the directory cannot be made.

## Ejemplo

```ahk
DirCreate "out"
```

## Relacionados

[`DirDelete`](DirDelete.md), [`DirCopy`](DirCopy.md)
