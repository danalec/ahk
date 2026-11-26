# `A_MSec`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · variable · 日期与时间

## 描述

机器本地时间的毫秒，三位数字。v2 的时间数字不带毫秒，而这个名字带。

## 语法

```ahk
A_MSec 
```

## 参数

nothing; it is a variable, not a call

## 返回值

毫秒，三位数字的文本

## 错误

the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

## 示例

```ahk
FileAppend A_MSec, "*"
```
