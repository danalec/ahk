# `DirCopy`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · command · 檔案與目錄操作

## 描述

Copies a directory and everything in it. What is copied is the content of the directory into the destination.

## 語法

```ahk
DirCopy …
```

## 參數

Source, Dest, [Overwrite]

## 傳回值

nothing

## 錯誤

Error `Failed` when the source is not a directory, or when the destination is there and the overwrite is not asked for; a name that holds a wildcard is refused, and the reference fails with `Failed` on a wildcard too.

## 範例

```ahk
DirCopy "src", "dst"
```
