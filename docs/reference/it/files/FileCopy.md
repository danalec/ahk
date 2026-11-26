# `FileCopy`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Operazioni su file e directory

## Descrizione

Copies a file, or every file that a pattern names. A destination that ends with a separator, or that is a folder, is the folder the files are copied into.

## Sintassi

```ahk
FileCopy Source, Dest, Overwrite
```

## Parametri

- **Source** — Path/wildcard.
- **Dest** — Destination.
- **Overwrite** — 1 to overwrite.

## Valore di ritorno

nothing

## Errori

Error `Failed` when the source is not there, or when the destination is there and the overwrite is not asked for.

## Esempio

```ahk
FileCopy "a.txt", "b.txt"
```

## Correlati

[`FileMove`](FileMove.md), [`DirCopy`](DirCopy.md)
