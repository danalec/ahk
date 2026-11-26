# `DirExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · command · ファイルとディレクトリの操作

## 説明

Answers the attributes of the first folder that the name or the pattern matches.

## 構文

```ahk
A := DirExist(Path)
```

## パラメータ

- **Path** — A path or wildcard.

## 戻り値

`D` when the folder is there, and the empty text when it is not; a file that matches the pattern is not a folder.

## エラー

nothing fails.

## 例

```ahk
x := DirExist("out")
```

## 関連

[`FileExist`](FileExist.md)
