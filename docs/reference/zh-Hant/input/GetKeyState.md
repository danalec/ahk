# `GetKeyState`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · 熱鍵、熱字串與輸入

## 描述

Reads what the hotkey watcher saw: whether a key is down. The second parameter carries only the words 'T' and 'P' of v2.

## 語法

```ahk
S := GetKeyState(KeyName, Mode)
```

## 參數

- **KeyName** — The key.
- **Mode** — P physical, T toggle.

## 傳回值

one when the key is down, zero when it is not

## 錯誤

a key name the tree does not know, and a mode that is neither 'T' nor 'P', are refused as invalid parameters; on a host that is not Linux the watcher cannot answer at all.

## 範例

```ahk
state := GetKeyState("Shift")
```

## 相關

[`KeyWait`](KeyWait.md), [`SetCapsLockState`](SetCapsLockState.md)
