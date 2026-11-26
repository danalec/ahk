# `Hotstring`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 热键、热字符串与输入

## 描述

注册一段文本，输入它时触发一个函数。v2 的文本写在两个相同字符之间，这两个字符不属于文本本身。

## 语法

```ahk
Hotstring …
```

## 参数

Text, Action [, Options]

## 返回值

无返回值

## 错误

a text that is not a string is refused as an invalid parameter; the watcher reads the physical keyboard, and the device of the box is skipped so that a hotstring does not fire on what the script types.

## 示例

```ahk
Hotstring "::btw::", ShowTheDefinition
```
