# `DirExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · command · Dosya ve dizin işlemleri

## Açıklama

Answers the attributes of the first folder that the name or the pattern matches.

## Sözdizimi

```ahk
A := DirExist(Path)
```

## Parametreler

- **Path** — A path or wildcard.

## Dönüş değeri

`D` when the folder is there, and the empty text when it is not; a file that matches the pattern is not a folder.

## Hatalar

nothing fails.

## Örnek

```ahk
x := DirExist("out")
```

## İlgili

[`FileExist`](FileExist.md)
