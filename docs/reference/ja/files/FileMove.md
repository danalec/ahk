# `FileMove`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · ファイルとディレクトリの操作

## 説明

Moves a file, or every file that a pattern names. The source and the destination are the ones of `FileCopy`.

## 構文

```ahk
FileMove Source, Dest, Overwrite
```

## パラメータ

- **Source** — Path/wildcard.
- **Dest** — Destination.
- **Overwrite** — 1 to overwrite.

## 戻り値

nothing

## エラー

Error `Failed` when the source is not there, or when the destination is there and the overwrite is not asked for.

## 例

```ahk
FileMove "a.txt", "b.txt"
```

## 関連

[`FileCopy`](FileCopy.md), [`DirMove`](DirMove.md)
