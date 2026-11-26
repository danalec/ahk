# `DirCreate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · command · 檔案與目錄操作

## 描述

Makes a directory, with the folders that are missing in the way.

## 語法

```ahk
DirCreate Path
```

## 參數

- **Path** — The directory.

## 傳回值

nothing

## 錯誤

OSError when the directory cannot be made.

## 範例

```ahk
DirCreate "out"
```

## 相關

[`DirDelete`](DirDelete.md), [`DirCopy`](DirCopy.md)
