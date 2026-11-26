# `MsgBox`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · GUI 窗口与窗口自动化

## 描述

显示一条消息。这棵树没有自己的窗口，因此文本进入报告的输出，而不是显示在屏幕上；调用返回同样的文本。

## 语法

```ahk
Result := MsgBox(Text, Title, Options)
```

## 参数

- **Text** — The message. If omitted, an empty box.
- **Title** — The title of the window. Defaults to the script's name.
- **Options** — A number or a text of icons/buttons: icon (16/32/48/64), buttons (0..5), default button, modality, timeout, and the owner.

## 返回值

消息的文本

## 错误

more than one parameter is refused with "the options, the title and the variable that takes the answer are not carried yet."

## 备注

On the Linux box the tree has no window of its own in a headless run: the text goes to the report output and the call answers the text. On Windows it is the MessageBox of user32. A timeout answers Timeout.

## 示例

```ahk
MsgBox "hello"
```

## 相关

[`InputBox`](InputBox.md), [`ToolTip`](ToolTip.md), [`TrayTip`](TrayTip.md)
