# `FileCopy`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Operacje na plikach i katalogach

## Opis

Copies a file, or every file that a pattern names. A destination that ends with a separator, or that is a folder, is the folder the files are copied into.

## Składnia

```ahk
FileCopy Source, Dest, Overwrite
```

## Parametry

- **Source** — Path/wildcard.
- **Dest** — Destination.
- **Overwrite** — 1 to overwrite.

## Wartość zwracana

nothing

## Błędy

Error `Failed` when the source is not there, or when the destination is there and the overwrite is not asked for.

## Przykład

```ahk
FileCopy "a.txt", "b.txt"
```

## Powiązane

[`FileMove`](FileMove.md), [`DirCopy`](DirCopy.md)
