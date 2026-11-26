# `StringGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 檔案與目錄操作

## 描述

The position of a substring (v1, 0-based).

## 語法

```ahk
StringGetPos Out, Haystack, Needle
```

## 參數

3..255.

## 傳回值

Nothing; 0-based pos or -1.

## 範例

```ahk
StringGetPos p, s, "x"
```

## 相關

[`InStr`](InStr.md)
