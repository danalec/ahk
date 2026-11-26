# `DateDiff`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · 日期与时间

## 描述

给出两个时刻之间的差；答案是第一个日期减第二个日期，向零截断。

## 语法

```ahk
DateDiff …
```

## 参数

DateTime1, DateTime2, Unit

## 返回值

差值，整数，单位是所要求的那个

## 错误

"Parameter #3 of DateDiff is invalid." when the unit is not one of d, h, m and s; a date that v2 cannot read is an invalid parameter.

## 示例

```ahk
days := DateDiff(20240110, 20240101, "d")
```
