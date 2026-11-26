# `Sleep`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 解析器与脚本执行

## 描述

让脚本暂停指定的毫秒数。

## 语法

```ahk
Sleep Milliseconds
```

## 参数

- **Milliseconds** — 0..60000; a larger value is clamped to 60000, a negative one sleeps 0.

## 返回值

等待的毫秒数

## 错误

none; a wait longer than 60000 milliseconds is cut at that number.

## 备注

The pause is clamped, not refused. During the sleep the timers of the script may still run (the runtime checks them between statements).

## 示例

```ahk
Sleep 500
```

## 相关

[`SetTimer`](SetTimer.md), [`Critical`](Critical.md)
