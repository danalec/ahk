# `DirExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · command · Operacje na plikach i katalogach

## Opis

Answers the attributes of the first folder that the name or the pattern matches.

## Składnia

```ahk
A := DirExist(Path)
```

## Parametry

- **Path** — A path or wildcard.

## Wartość zwracana

`D` when the folder is there, and the empty text when it is not; a file that matches the pattern is not a folder.

## Błędy

nothing fails.

## Przykład

```ahk
x := DirExist("out")
```

## Powiązane

[`FileExist`](FileExist.md)
