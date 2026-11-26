# `SplitPath`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Dosya ve dizin işlemleri

## Açıklama

Cuts a path into its pieces and writes them into the variables of the caller. A path of a file is cut at the backslash and a url at the slash, and the drive of a url is its scheme and its host.

## Sözdizimi

```ahk
SplitPath …
```

## Parametreler

Path, &OutFileName, &OutDir, &OutExt, &OutNameNoExt, &OutDrive

## Dönüş değeri

nothing

## Hatalar

TypeError when a piece is not a variable to write into.

## Örnek

```ahk
SplitPath "C:\dir\file.txt", &name, &dir, &ext
```
