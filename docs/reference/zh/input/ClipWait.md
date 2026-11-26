# `ClipWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 热键、热字符串与输入

## 描述

Waits for the clipboard to hold data.

## 语法

```ahk
ClipWait Timeout, WaitForAny
```

## 参数

0..255.

## 返回值

Nothing; throws TimeoutError.

## 示例

```ahk
ClipWait 2
```

## 相关

[`ClipboardAll`](ClipboardAll.md), [`A_Clipboard`](A_Clipboard.md)
