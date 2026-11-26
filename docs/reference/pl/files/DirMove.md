# `DirMove`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · command · Operacje na plikach i katalogach

## Opis

Moves a directory, with everything in it. A destination that is there is merged with the one that comes: the files of the source win.

## Składnia

```ahk
DirMove …
```

## Parametry

Source, Dest, [Overwrite]

## Wartość zwracana

nothing

## Błędy

Error `Failed` when the source is not a directory; a name that holds a wildcard is refused, and the reference fails with `Failed` on a wildcard too.

## Przykład

```ahk
DirMove "src", "dst"
```
