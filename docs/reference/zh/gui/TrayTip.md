# `TrayTip`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · GUI 窗口与窗口自动化

## 描述

向桌面发送一条带文本和标题的通知。这个名字来自 Windows（通知区里的气泡），但这棵树发送的是桌面通知，同样的用户会看到同样的内容。

## 语法

```ahk
TrayTip Text, Title, Options
```

## 参数

- **Text** — The message.
- **Title** — The title.
- **Options** — Icon/mute.

## 返回值

空文本

## 错误

the desktop reports the failure of the notification.

## 示例

```ahk
TrayTip "done", "the script"
```

## 相关

[`ToolTip`](ToolTip.md), [`MsgBox`](MsgBox.md)
