# `GetKeyName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · 热键、热字符串与输入

## 描述

按键的名称，按 v2 的写法。名称取自 v2 的按键表：`Esc`、`Enter`、`CtrlRight`、`Numpad0`，以及 `vkXX` 和 `scXXX` 写法。

## 语法

```ahk
GetKeyName …
```

## 参数

KeyName

## 返回值

按键的名称

## 错误

a name the tree does not know is the empty text, the way v2 answers a name it does not know.

## 示例

```ahk
name := GetKeyName("Enter")
```
