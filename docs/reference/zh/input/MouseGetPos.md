# `MouseGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 热键、热字符串与输入

## 描述

从内核读取本运行时所驱动的指针位置。它是这台机器上唯一能读出位置的指针：物理鼠标是相对设备。

## 语法

```ahk
MouseGetPos OutputX, OutputY, OutputWin, OutputControl, Flag
```

## 参数

- **Output*** — Variables that receive the answers.

## 返回值

无返回值；位置写入调用者的变量

## 错误

when the kernel will not answer, the desktop answers the place the runtime last put the pointer at, and says so in a note.

## 示例

```ahk
MouseGetPos &x, &y
```

## 相关

[`MouseMove`](MouseMove.md), [`PixelGetColor`](PixelGetColor.md)
