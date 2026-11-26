# `GetKeyState`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 热键、热字符串与输入

## 描述

读取热键监视器所看到的状态：某个键是否按下。第二个参数只接受 v2 的 'T' 和 'P'。

## 语法

```ahk
S := GetKeyState(KeyName, Mode)
```

## 参数

- **KeyName** — The key.
- **Mode** — P physical, T toggle.

## 返回值

按下时为 1，否则为 0

## 错误

a key name the tree does not know, and a mode that is neither 'T' nor 'P', are refused as invalid parameters; on a host that is not Linux the watcher cannot answer at all.

## 示例

```ahk
state := GetKeyState("Shift")
```

## 相关

[`KeyWait`](KeyWait.md), [`SetCapsLockState`](SetCapsLockState.md)
