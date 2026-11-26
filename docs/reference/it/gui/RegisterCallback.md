# `RegisterCallback`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · function · Finestre GUI e automazione delle finestre

## Descrizione

Creates a machine-code callback address for a script function.

## Sintassi

```ahk
Addr := RegisterCallback(Function, Options, ParamCount)
```

## Parametri

- **Function** — The function.
- **Options** — CDecl/Fast/Indirect.
- **ParamCount** — Params.

## Valore di ritorno

An address; this tree refuses at run (it does not manufacture machine code).

## Esempio

```ahk
cb := RegisterCallback("MyFunc")
```

## Correlati

[`DllCall`](DllCall.md), [`CallbackCreate`](CallbackCreate.md)
