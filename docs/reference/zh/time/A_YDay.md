# `A_YDay`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · variable · 日期与时间

## 描述

本地时间中的年内第几天，1 月 1 日是第一天。

## 语法

```ahk
A_YDay 
```

## 参数

nothing; it is a variable, not a call

## 返回值

年内第几天，作为数字的文本

## 错误

the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

## 示例

```ahk
FileAppend A_YDay, "*"
```
