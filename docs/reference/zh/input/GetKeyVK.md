# `GetKeyVK`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · 热键、热字符串与输入

## 描述

按键的 Windows 虚拟键码。它是 Windows 键盘的按键编号，本树保留它是为了那些需要它的脚本。

## 语法

```ahk
GetKeyVK …
```

## 参数

KeyName

## 返回值

虚拟键码，一个数字

## 错误

a name the tree does not know is the empty text, the way v2 answers a name it does not know.

## 示例

```ahk
vk := GetKeyVK("Enter")
```
