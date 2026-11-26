# `SendInput`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · 热键、热字符串与输入

## 描述

按后端的方式发送按键：v2 的输入方式就是后端键盘的方式，因此按键走的路径与 `Send` 相同。

## 语法

```ahk
SendInput …
```

## 参数

Keys

## 返回值

无返回值

## 错误

the keyboard of the backend is a Linux uinput device, and it fails on a host that is not Linux.

## 示例

```ahk
SendInput "{Enter}"
```
