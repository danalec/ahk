# `Pause`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 解析器与脚本执行

## 描述

设置脚本的暂停状态。这棵树只记录状态，而不是等待按键——没有键盘的脚本会因此挂住。状态可在 A_IsPaused 中看到。

## 语法

```ahk
Pause …
```

## 参数

[OnOff]

## 返回值

空文本

## 错误

a parameter that is not 0, 1 or -1 is an invalid parameter.

## 示例

```ahk
Pause true
```
