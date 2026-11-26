# `FileGetSize`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 檔案與目錄操作

## 描述

Answers how big a file is.

## 語法

```ahk
FileGetSize …
```

## 參數

FileName, [Unit]

## 傳回值

the size, rounded down; a folder answers zero.

## 錯誤

ValueError when the unit is not `B`, `K` or `M`; OSError when the file cannot be read.

## 範例

```ahk
n := FileGetSize("out.txt", "K")
```
