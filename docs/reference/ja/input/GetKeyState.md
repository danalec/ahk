# `GetKeyState`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · ホットキー、ホット文字列、入力

## 説明

Reads what the hotkey watcher saw: whether a key is down. The second parameter carries only the words 'T' and 'P' of v2.

## 構文

```ahk
S := GetKeyState(KeyName, Mode)
```

## パラメータ

- **KeyName** — The key.
- **Mode** — P physical, T toggle.

## 戻り値

one when the key is down, zero when it is not

## エラー

a key name the tree does not know, and a mode that is neither 'T' nor 'P', are refused as invalid parameters; on a host that is not Linux the watcher cannot answer at all.

## 例

```ahk
state := GetKeyState("Shift")
```

## 関連

[`KeyWait`](KeyWait.md), [`SetCapsLockState`](SetCapsLockState.md)
