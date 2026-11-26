# `A_Cursor`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · variable · 热键、热字符串与输入

## 描述

指针光标的形状。形状属于机器，而机器是被询问的。

## 语法

```ahk
A_Cursor 
```

## 参数

nothing; it is a variable, not a call

## 返回值

形状的名称，作为文本（"Arrow"、"IBeam"、"Wait" 等）

## 错误

the read fails with the reason of the desktop when the desktop cannot tell the shape of the cursor; the write is refused, because the name is a built-in variable that the dialect does not let a script write.

## 示例

```ahk
FileAppend A_Cursor, "*"
```
