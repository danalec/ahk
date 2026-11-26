# `FileMove`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 檔案與目錄操作

## 描述

Moves a file, or every file that a pattern names. The source and the destination are the ones of `FileCopy`.

## 語法

```ahk
FileMove Source, Dest, Overwrite
```

## 參數

- **Source** — Path/wildcard.
- **Dest** — Destination.
- **Overwrite** — 1 to overwrite.

## 傳回值

nothing

## 錯誤

Error `Failed` when the source is not there, or when the destination is there and the overwrite is not asked for.

## 範例

```ahk
FileMove "a.txt", "b.txt"
```

## 相關

[`FileCopy`](FileCopy.md), [`DirMove`](DirMove.md)
