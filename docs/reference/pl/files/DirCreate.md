# `DirCreate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · command · Operacje na plikach i katalogach

## Opis

Makes a directory, with the folders that are missing in the way.

## Składnia

```ahk
DirCreate Path
```

## Parametry

- **Path** — The directory.

## Wartość zwracana

nothing

## Błędy

OSError when the directory cannot be made.

## Przykład

```ahk
DirCreate "out"
```

## Powiązane

[`DirDelete`](DirDelete.md), [`DirCopy`](DirCopy.md)
