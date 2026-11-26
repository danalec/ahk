# `ControlGetFocus`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · GUI 窗口与窗口自动化

## 描述

Reads the focused control.

## 语法

```ahk
S := ControlGetFocus(WinTitle)
```

## 参数

1..255.

## 返回值

The ClassNN.

## 示例

```ahk
ControlGetFocus("A")
```

## 相关

[`ControlSend`](ControlSend.md), [`Gui`](Gui.md)
