# `DirCreate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · command · ファイルとディレクトリの操作

## 説明

Makes a directory, with the folders that are missing in the way.

## 構文

```ahk
DirCreate Path
```

## パラメータ

- **Path** — The directory.

## 戻り値

nothing

## エラー

OSError when the directory cannot be made.

## 例

```ahk
DirCreate "out"
```

## 関連

[`DirDelete`](DirDelete.md), [`DirCopy`](DirCopy.md)
