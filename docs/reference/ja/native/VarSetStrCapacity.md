# `VarSetStrCapacity`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · function · ネイティブ関数の呼び出し (DllCall)

## 説明

Sets the capacity of a variable's string buffer (or frees it).

## 構文

```ahk
VarSetStrCapacity Var, Capacity
```

## パラメータ

- **Var** — The variable.
- **Capacity** — Chars; -1 frees.

## 戻り値

Nothing.

## 例

```ahk
VarSetStrCapacity s, 1024
```

## 関連

[`StrLen`](StrLen.md), [`Buffer`](Buffer.md)
