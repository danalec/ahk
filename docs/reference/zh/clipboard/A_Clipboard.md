# `A_Clipboard`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · variable · 剪贴板

## 描述

桌面剪贴板中的文本。脚本可以写它，连接运算会在已有内容后追加。

## 语法

```ahk
A_Clipboard 
```

## 参数

nothing; it is a variable, not a call

## 返回值

剪贴板中的文本

## 错误

the read fails with the reason of the desktop; the write of a number is refused, because the clipboard holds a text.

## 示例

```ahk
A_Clipboard := "hello"
FileAppend A_Clipboard, "*"
```
