# `MouseMove`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 热键、热字符串与输入

## 描述

把后端的指针移动到屏幕上的某个位置。后端是把指针移到该位置，而不是从当前位置相对移动。

## 语法

```ahk
MouseMove X, Y, Speed
```

## 参数

- **X/Y** — Screen coords (or relative with R).
- **Speed** — 0..100.

## 返回值

无返回值

## 错误

the relative place of the move and the speed of the move are refused as not carried yet.

## 示例

```ahk
MouseMove 400, 300
```

## 相关

[`Click`](Click.md), [`MouseGetPos`](MouseGetPos.md)
