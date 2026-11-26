# `ToolTip`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · GUI 窗口与窗口自动化

## 描述

在屏幕的某个位置显示一个小文本窗口。不带文本调用时会把窗口移除。

## 语法

```ahk
ToolTip Text, X, Y, WhichToolTip
```

## 参数

- **Text** — The text; omitted hides.
- **X/Y** — Position.
- **WhichToolTip** — 1..20.

## 返回值

标识该窗口的编号；移除窗口时返回空文本

## 错误

the desktop reports the failure of the call.

## 示例

```ahk
ToolTip "hello", 10, 10
```

## 相关

[`TrayTip`](TrayTip.md), [`MsgBox`](MsgBox.md)
