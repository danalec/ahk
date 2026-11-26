# `FileDelete`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Operaciones de archivos y directorios

## Descripción

Takes a file away, or every file that a pattern names. A pattern that matches nothing is not a failure.

## Sintaxis

```ahk
FileDelete Pattern
```

## Parámetros

- **Pattern** — A path or wildcard.

## Valor de retorno

nothing

## Errores

OSError when the file cannot be taken away.

## Ejemplo

```ahk
FileDelete "out.txt"
```

## Relacionados

[`DirDelete`](DirDelete.md), [`FileRecycle`](FileRecycle.md)
