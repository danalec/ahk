# `DirCreate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · command · Dosya ve dizin işlemleri

## Açıklama

Makes a directory, with the folders that are missing in the way.

## Sözdizimi

```ahk
DirCreate Path
```

## Parametreler

- **Path** — The directory.

## Dönüş değeri

nothing

## Hatalar

OSError when the directory cannot be made.

## Örnek

```ahk
DirCreate "out"
```

## İlgili

[`DirDelete`](DirDelete.md), [`DirCopy`](DirCopy.md)
