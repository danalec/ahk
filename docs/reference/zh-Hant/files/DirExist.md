# `DirExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · command · 檔案與目錄操作

## 描述

Answers the attributes of the first folder that the name or the pattern matches.

## 語法

```ahk
A := DirExist(Path)
```

## 參數

- **Path** — A path or wildcard.

## 傳回值

`D` when the folder is there, and the empty text when it is not; a file that matches the pattern is not a folder.

## 錯誤

nothing fails.

## 範例

```ahk
x := DirExist("out")
```

## 相關

[`FileExist`](FileExist.md)
