# `FormatTime`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 日期与时间

## 描述

按格式写出的时刻。日期在前，格式在后；完全不含日期的文本表示机器当前的时刻。

## 语法

```ahk
S := FormatTime(YYYYMMDDHH24MISS, Format)
```

## 参数

- **Time** — A timestamp; omitted = now.
- **Format** — The picture (yyyy-MM-dd etc.).

## 返回值

按格式写出的日期

## 错误

a date the dialect cannot read is an invalid parameter.

## 示例

```ahk
FormatTime(A_Now, "yyyy-MM-dd")
```

## 相关

[`DateAdd`](DateAdd.md), [`DateDiff`](DateDiff.md), [`A_Now`](A_Now.md)
