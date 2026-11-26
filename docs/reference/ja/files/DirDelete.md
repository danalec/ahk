# `DirDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · command · ファイルとディレクトリの操作

## 説明

Takes a directory away. A directory that is not empty stays without it unless the recursion is asked for.

## 構文

```ahk
DirDelete Path, Recurse
```

## パラメータ

- **Path** — The directory.
- **Recurse** — 1 to delete contents.

## 戻り値

nothing; the name of a command answers nothing.

## エラー

Error `Failed` when the directory is not there, or when it is not empty and the recursion is not asked for.

## 例

```ahk
d := DirDelete("out", 1)
```

## 関連

[`DirCreate`](DirCreate.md), [`FileDelete`](FileDelete.md)
