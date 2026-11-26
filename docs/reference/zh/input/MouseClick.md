# `MouseClick`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 热键、热字符串与输入

## 描述

用后端的指针点击。按钮是第一个参数，与 v2 函数的写法一致；不写按钮时是左键。

## 语法

```ahk
MouseClick …
```

## 参数

Button, X, Y [Count] [Options]

## 返回值

无返回值

## 错误

the wheel, the fourth and the fifth buttons are refused as not carried yet.

## 示例

```ahk
MouseClick "Right", 100, 200
```
