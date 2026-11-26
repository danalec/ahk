# `WinWaitClose`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · GUI 窗口与窗口自动化

## 描述

等待标题匹配的窗口消失，并回答它是否已经消失。

## 语法

```ahk
WinWaitClose …
```

## 参数

WinTitle, WinText, [Timeout], [ExcludeTitle], [ExcludeText]

## 返回值

窗口已消失为 1，超时时它还在则为 0

## 错误

a session that is not Wayland answers "this desktop waits for the windows of a Wayland session, and this session is not Wayland"; the two exclusions are refused with "the two exclusions are not carried yet."

## 示例

```ahk
WinWaitClose "Calculator", , 5
```
