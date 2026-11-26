# `A_ScreenHeight`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · variable · 屏幕捕获与像素函数

## 描述

桌面屏幕的高度，单位是像素。

## 语法

```ahk
A_ScreenHeight 
```

## 参数

nothing; it is a variable, not a call

## 返回值

高度，作为数字

## 错误

the read fails with the reason of the desktop when the screen cannot be measured; the write is refused.

## 示例

```ahk
FileAppend A_ScreenHeight, "*"
```
