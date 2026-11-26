# `RegisterCallback`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · GUI 視窗與視窗自動化

## 描述

Creates a machine-code callback address for a script function.

## 語法

```ahk
Addr := RegisterCallback(Function, Options, ParamCount)
```

## 參數

- **Function** — The function.
- **Options** — CDecl/Fast/Indirect.
- **ParamCount** — Params.

## 傳回值

An address; this tree refuses at run (it does not manufacture machine code).

## 範例

```ahk
cb := RegisterCallback("MyFunc")
```

## 相關

[`DllCall`](DllCall.md), [`CallbackCreate`](CallbackCreate.md)
