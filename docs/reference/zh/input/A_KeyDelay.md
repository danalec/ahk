# `A_KeyDelay`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · variable · 热键、热字符串与输入

## 描述

`Send` 发送按键之间的延迟，单位是毫秒。它是运行的设置项，默认是十。

## 语法

```ahk
A_KeyDelay 
```

## 参数

nothing; it is a variable, not a call

## 返回值

延迟，作为数字

## 错误

the write takes the number of the value, and a text that holds no number is zero.

## 示例

```ahk
FileAppend A_KeyDelay, "*"
```
