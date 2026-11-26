# `RegisterCallback`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · function · GUI windows and window automation

## Description

Creates a machine-code callback address for a script function.

## Syntax

```ahk
Addr := RegisterCallback(Function, Options, ParamCount)
```

## Parameters

- **Function** — The function.
- **Options** — CDecl/Fast/Indirect.
- **ParamCount** — Params.

## Return value

An address; this tree refuses at run (it does not manufacture machine code).

## Example

```ahk
cb := RegisterCallback("MyFunc")
```

## Related

[`DllCall`](DllCall.md), [`CallbackCreate`](CallbackCreate.md)
