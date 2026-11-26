# `DirDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · command · Operacje na plikach i katalogach

## Opis

Takes a directory away. A directory that is not empty stays without it unless the recursion is asked for.

## Składnia

```ahk
DirDelete Path, Recurse
```

## Parametry

- **Path** — The directory.
- **Recurse** — 1 to delete contents.

## Wartość zwracana

nothing; the name of a command answers nothing.

## Błędy

Error `Failed` when the directory is not there, or when it is not empty and the recursion is not asked for.

## Przykład

```ahk
d := DirDelete("out", 1)
```

## Powiązane

[`DirCreate`](DirCreate.md), [`FileDelete`](FileDelete.md)
