# `WinMaximize`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · GUI 窗口与窗口自动化

## 描述

最大化脚本自己创建的窗口，且只有这一个；由盒子里的辅助程序来完成。

## 语法

```ahk
WinMaximize …
```

## 参数

WinTitle

## 返回值

空文本

## 错误

"Target window not found." when the script has no window of its own, or when the title does not match the one it has.

## 示例

```ahk
WinMaximize "MyWindow"
```
