# `WinGetTitle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · GUI 窗口与窗口自动化

## 描述

取得活动窗口的标题，或与给定文本匹配的窗口的标题。

## 语法

```ahk
T := WinGetTitle(WinTitle, WinText)
```

## 参数

- **WinTitle** — The criteria; omitted = last found.

## 返回值

窗口的标题

## 错误

a window that was named and is not there answers 'Target window not found.'; the text of the window and the exclusions are not carried yet.

## 示例

```ahk
title := WinGetTitle()
```

## 相关

[`WinGetText`](WinGetText.md), [`WinGetPos`](WinGetPos.md)
