# `WinSetStyle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · GUI 窗口与窗口自动化

## 描述

写入窗口的样式。样式是绝对的：文本在数字之前可以带 `+`、`-` 或 `^`，运行时就已有的样式算出新样式——`+0x10` 加上这些位，`-0x10` 去掉它们，`^0x10` 把它们翻转，没有前缀的文本则替换样式。数字按 v2 的 `ATOU` 读取：文本写着 "0x" 时是十六进制，否则是十进制，文本里没有数字时是零。

## 语法

```ahk
WinSetStyle …
```

## 参数

Style, WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## 返回值

空文本

## 错误

an empty text refuses with 'Parameter #1 of WinSetStyle is invalid.', and 'empty string' is in the `Extra`; a text that holds no number is not a refusal -- it is read as zero, and the failure that comes is the one of the window.

## 示例

```ahk
WinSetStyle("+0x10", "ZZZ_NO_WINDOW_ZZZ")
```
