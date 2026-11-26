# `ControlGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 热键、热字符串与输入

## 描述

Reads a control property (v1).

## 语法

```ahk
ControlGet Out, Cmd, Value, Control, WinTitle
```

## 参数

2..255.

## 返回值

Nothing.

## 示例

```ahk
ControlGet t, "Text", , "Edit1"
```

## 相关

[`ControlSend`](ControlSend.md), [`Gui`](Gui.md)
