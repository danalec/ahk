# `ControlSetExStyle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · GUI 窗口与窗口自动化

## 描述

写入窗口中一个控件的扩展样式。样式是绝对的，与窗口的扩展样式一样：文本在数字之前可以带 `+`、`-` 或 `^`，运行时就已有的扩展样式算出新的扩展样式。

## 语法

```ahk
ControlSetExStyle …
```

## 参数

Style, Control, WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## 返回值

空文本

## 错误

an empty text refuses with 'Parameter #1 of ControlSetExStyle is invalid.'; a window that is not there refuses with 'Target window not found.', and a control that is not there with 'Target control not found.'.

## 示例

```ahk
ControlSetExStyle("+0x8", "Edit1", "ZZZ_NO_WINDOW_ZZZ")
```
