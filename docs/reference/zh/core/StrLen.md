# `StrLen`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 语言核心：文本、数学与模式

## 描述

按 UTF-16 码元统计文本的字符数，因此基本平面之外的字符算作两个。

## 语法

```ahk
Count := StrLen(String)
```

## 参数

- **String** — The text.

## 返回值

码元的数量

## 错误

none; a value that is not text is read as its text.

## 备注

Astral characters count as one each on this tree.

## 示例

```ahk
StrLen("hello")
```

## 相关

[`SubStr`](SubStr.md), [`InStr`](InStr.md)
