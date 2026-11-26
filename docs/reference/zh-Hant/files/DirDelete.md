# `DirDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · command · 檔案與目錄操作

## 描述

Takes a directory away. A directory that is not empty stays without it unless the recursion is asked for.

## 語法

```ahk
DirDelete Path, Recurse
```

## 參數

- **Path** — The directory.
- **Recurse** — 1 to delete contents.

## 傳回值

nothing; the name of a command answers nothing.

## 錯誤

Error `Failed` when the directory is not there, or when it is not empty and the recursion is not asked for.

## 範例

```ahk
d := DirDelete("out", 1)
```

## 相關

[`DirCreate`](DirCreate.md), [`FileDelete`](FileDelete.md)
