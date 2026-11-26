# `FileDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 檔案與目錄操作

## 描述

Takes a file away, or every file that a pattern names. A pattern that matches nothing is not a failure.

## 語法

```ahk
FileDelete Pattern
```

## 參數

- **Pattern** — A path or wildcard.

## 傳回值

nothing

## 錯誤

OSError when the file cannot be taken away.

## 範例

```ahk
FileDelete "out.txt"
```

## 相關

[`DirDelete`](DirDelete.md), [`FileRecycle`](FileRecycle.md)
