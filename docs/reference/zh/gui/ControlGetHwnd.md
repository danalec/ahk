# `ControlGetHwnd`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · GUI 窗口与窗口自动化

## 描述

Reads the handle of a control.

## 语法

```ahk
H := ControlGetHwnd(Control, WinTitle)
```

## 参数

1..5.

## 返回值

The handle.

## 示例

```ahk
ControlGetHwnd("Edit1")
```

## 相关

[`ControlSend`](ControlSend.md), [`Gui`](Gui.md)
