# `SplitPath`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · ファイルとディレクトリの操作

## 説明

Cuts a path into its pieces and writes them into the variables of the caller. A path of a file is cut at the backslash and a url at the slash, and the drive of a url is its scheme and its host.

## 構文

```ahk
SplitPath …
```

## パラメータ

Path, &OutFileName, &OutDir, &OutExt, &OutNameNoExt, &OutDrive

## 戻り値

nothing

## エラー

TypeError when a piece is not a variable to write into.

## 例

```ahk
SplitPath "C:\dir\file.txt", &name, &dir, &ext
```
