# `KeyWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · ホットキー、ホット文字列、入力

## 説明

Refused. The runtime does not keep the keys that are down, so it cannot wait for one.

## 構文

```ahk
KeyWait KeyName, Options
```

## パラメータ

- **KeyName** — The key.
- **Options** — D (down), T timeout, L logical.

## 戻り値

nothing; the statement fails

## エラー

this runtime does not keep the keys that are down, so it cannot wait for one.

## 例

```ahk
KeyWait "a"
```

## 関連

[`GetKeyState`](GetKeyState.md), [`Hotkey`](Hotkey.md)
