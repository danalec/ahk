# `WinGetClientPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · GUI 窗口与窗口自动化

## 描述

取得窗口中程序绘制的那一部分的位置与大小。Wayland 客户端不自己画边框——边框由合成器画——因此合成器报告的几何就是用户看到的窗口，两条命令的回答相同。

## 语法

```ahk
WinGetClientPos …
```

## 参数

[WindowTitle] [, WinText, &X, &Y, &Width, &Height, ExcludeTitle, ExcludeText]

## 返回值

无返回值；位置和大小写入调用者的变量

## 错误

the text of the window and the exclusions are not carried yet; a window that is not there answers 'Target window not found.'

## 示例

```ahk
WinGetClientPos "Untitled", , &x, &y, &w, &h
```
