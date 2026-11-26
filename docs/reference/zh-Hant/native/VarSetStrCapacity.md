# `VarSetStrCapacity`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · function · 原生函式呼叫 (DllCall)

## 描述

Sets the capacity of a variable's string buffer (or frees it).

## 語法

```ahk
VarSetStrCapacity Var, Capacity
```

## 參數

- **Var** — The variable.
- **Capacity** — Chars; -1 frees.

## 傳回值

Nothing.

## 範例

```ahk
VarSetStrCapacity s, 1024
```

## 相關

[`StrLen`](StrLen.md), [`Buffer`](Buffer.md)
