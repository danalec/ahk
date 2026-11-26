# `WinGetClass`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · GUI 窗口与窗口自动化

## 描述

取得与给定标题匹配的窗口的类。

## 语法

```ahk
WinGetClass …
```

## 参数

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## 返回值

窗口的类

## 错误

a title that matches no window answers 'Target window not found.'; the text of the window and the exclusions are not carried yet.

## 示例

```ahk
class := WinGetClass("Untitled")
```
