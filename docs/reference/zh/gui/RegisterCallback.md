# `RegisterCallback`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · GUI 窗口与窗口自动化

## 描述

Creates a machine-code callback address for a script function.

## 语法

```ahk
Addr := RegisterCallback(Function, Options, ParamCount)
```

## 参数

- **Function** — The function.
- **Options** — CDecl/Fast/Indirect.
- **ParamCount** — Params.

## 返回值

An address; this tree refuses at run (it does not manufacture machine code).

## 示例

```ahk
cb := RegisterCallback("MyFunc")
```

## 相关

[`DllCall`](DllCall.md), [`CallbackCreate`](CallbackCreate.md)
