# `SendText`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · 热键、热字符串与输入

## 描述

按原样输入文本：这里不解析 v2 的大括号，文本逐字符送入键盘。

## 语法

```ahk
SendText …
```

## 参数

Text

## 返回值

无返回值

## 错误

the keyboard of the backend is a Linux uinput device, so on a host that is not Linux the text is not typed.

## 示例

```ahk
SendText "a literal {braced} text"
```
