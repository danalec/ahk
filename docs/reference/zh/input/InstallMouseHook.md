# `InstallMouseHook`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · 热键、热字符串与输入

## 描述

回答钩子已经就位。鼠标完全没有被监视：后端监视的是键盘，而指针只是一个写入、从不读取的设备。

## 语法

```ahk
InstallMouseHook …
```

## 参数

[OnOff] [HookName]

## 返回值

无返回值

## 错误

none; the command never fails.

## 示例

```ahk
InstallMouseHook
```
