# `DirMove`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · command · 檔案與目錄操作

## 描述

Moves a directory, with everything in it. A destination that is there is merged with the one that comes: the files of the source win.

## 語法

```ahk
DirMove …
```

## 參數

Source, Dest, [Overwrite]

## 傳回值

nothing

## 錯誤

Error `Failed` when the source is not a directory; a name that holds a wildcard is refused, and the reference fails with `Failed` on a wildcard too.

## 範例

```ahk
DirMove "src", "dst"
```
