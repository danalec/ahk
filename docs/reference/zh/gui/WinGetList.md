# `WinGetList`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · GUI 窗口与窗口自动化

## 描述

取得所有与给定标题匹配的窗口的标识。

## 语法

```ahk
WinGetList …
```

## 参数

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## 返回值

标识的数组，没有匹配时为空数组

## 错误

the text of the window and the exclusions are not carried yet; a question the compositor cannot answer fails with its reason.

## 示例

```ahk
list := WinGetList("Untitled")
```
