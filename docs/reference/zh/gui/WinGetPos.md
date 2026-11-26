# `WinGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · GUI 窗口与窗口自动化

## 描述

取得窗口的位置和大小，数值来自合成器的报告。

## 语法

```ahk
WinGetPos X, Y, W, H, WinTitle
```

## 参数

- **Output*** — Variables.
- **WinTitle** — The criteria.

## 返回值

无返回值；位置和大小写入调用者的变量

## 错误

the text of the window and the exclusions are not carried yet; a window that is not there answers 'Target window not found.'

## 示例

```ahk
WinGetPos "Untitled", , &x, &y, &w, &h
```

## 相关

[`WinMove`](WinMove.md), [`WinGetTitle`](WinGetTitle.md)
