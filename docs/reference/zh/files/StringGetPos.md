# `StringGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 文件与目录操作

## 描述

The position of a substring (v1, 0-based).

## 语法

```ahk
StringGetPos Out, Haystack, Needle
```

## 参数

3..255.

## 返回值

Nothing; 0-based pos or -1.

## 示例

```ahk
StringGetPos p, s, "x"
```

## 相关

[`InStr`](InStr.md)
