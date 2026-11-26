# `WinWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · GUI 窗口与窗口自动化

## 描述

等待标题匹配的窗口出现，并给出它的标识符；Wayland 窗口的标识符是一个记号，所以是文本。

## 语法

```ahk
WinWait WinTitle, WinText, Timeout
```

## 参数

- **WinTitle** — The criteria.
- **Timeout** — Seconds; omitted waits forever.

## 返回值

窗口标识符（文本），超时则为 0

## 错误

a session that is not Wayland answers "this desktop waits for the windows of a Wayland session, and this session is not Wayland"; the two exclusions are refused with "the two exclusions are not carried yet."; the wait is capped at sixty seconds when the timeout is left out.

## 示例

```ahk
WinWait "Calculator", , 5
```

## 相关

[`WinExist`](WinExist.md), [`WinActivate`](WinActivate.md)
