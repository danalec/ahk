# `SysGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 机器、进程与环境

## 描述

给出系统的一个度量；编号取自 GetSystemMetrics，只有与屏幕有关的那些被承载。

## 语法

```ahk
SysGet …
```

## 参数

Metric

## 返回值

该度量（整数）

## 错误

a metric of the window manager of Windows is refused with "SysGet(n) is a metric of the window manager of Windows, and this desktop does not have one."; a compositor that published no monitor answers "the compositor published no monitor, so the metric n of SysGet is not known."; a number that is not a metric at all is zero.

## 示例

```ahk
w := SysGet(0)
```
