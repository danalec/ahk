# `FileDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Operacje na plikach i katalogach

## Opis

Takes a file away, or every file that a pattern names. A pattern that matches nothing is not a failure.

## Składnia

```ahk
FileDelete Pattern
```

## Parametry

- **Pattern** — A path or wildcard.

## Wartość zwracana

nothing

## Błędy

OSError when the file cannot be taken away.

## Przykład

```ahk
FileDelete "out.txt"
```

## Powiązane

[`DirDelete`](DirDelete.md), [`FileRecycle`](FileRecycle.md)
