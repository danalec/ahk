# `KeyWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 熱鍵、熱字串與輸入

## 描述

Refused. The runtime does not keep the keys that are down, so it cannot wait for one.

## 語法

```ahk
KeyWait KeyName, Options
```

## 參數

- **KeyName** — The key.
- **Options** — D (down), T timeout, L logical.

## 傳回值

nothing; the statement fails

## 錯誤

this runtime does not keep the keys that are down, so it cannot wait for one.

## 範例

```ahk
KeyWait "a"
```

## 相關

[`GetKeyState`](GetKeyState.md), [`Hotkey`](Hotkey.md)
