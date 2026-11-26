# `FileGetVersion`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 檔案與目錄操作

## 描述

The version of a file.

## 語法

```ahk
S := FileGetVersion(Filename)
```

## 參數

1..255.

## 傳回值

The version.

## 範例

```ahk
v := FileGetVersion("a.exe")
```

## 相關

[`FileExist`](FileExist.md)
