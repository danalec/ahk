# `KeyWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 热键、热字符串与输入

## 描述

被拒绝。本运行时不保存处于按下状态的按键，因此无法等待某个键。

## 语法

```ahk
KeyWait KeyName, Options
```

## 参数

- **KeyName** — The key.
- **Options** — D (down), T timeout, L logical.

## 返回值

无返回值；语句失败

## 错误

this runtime does not keep the keys that are down, so it cannot wait for one.

## 示例

```ahk
KeyWait "a"
```

## 相关

[`GetKeyState`](GetKeyState.md), [`Hotkey`](Hotkey.md)
