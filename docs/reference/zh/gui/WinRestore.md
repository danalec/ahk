# `WinRestore`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · GUI 窗口与窗口自动化

## 描述

把脚本自己的窗口从最小化状态恢复，且只有这一个窗口。

## 语法

```ahk
WinRestore …
```

## 参数

WinTitle

## 返回值

空文本

## 错误

"Target window not found." when the script has no window of its own, or when the title does not match the one it has.

## 示例

```ahk
WinRestore "MyWindow"
```
