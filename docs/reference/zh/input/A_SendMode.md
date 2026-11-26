# `A_SendMode`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · variable · 热键、热字符串与输入

## 描述

`Send` 使用的方法：`Input`、`Event`、`Play` 或 `InputThenPlay`。

## 语法

```ahk
A_SendMode 
```

## 参数

nothing; it is a variable, not a call

## 返回值

模式的名称，作为文本

## 错误

the write takes the name of one of the four modes, and any other name is refused with "Invalid value.".

## 示例

```ahk
FileAppend A_SendMode, "*"
```
