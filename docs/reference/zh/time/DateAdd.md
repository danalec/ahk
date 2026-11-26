# `DateAdd`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · 日期与时间

## 描述

给出一个时刻加上一段时间；日期在前、单位在后，单位只读第一个字母。

## 语法

```ahk
DateAdd …
```

## 参数

DateTime, Number, Unit

## 返回值

新的日期，文本形式 YYYYMMDDHH24MISS

## 错误

"Parameter #3 of DateAdd is invalid." when the unit is not one of d, h, m and s; "Parameter #1 of DateAdd is invalid." when the date is not one that v2 can read; "Parameter #2 of DateAdd requires a Number, but received a String."

## 示例

```ahk
next := DateAdd(20240101, 1, "d")
```
