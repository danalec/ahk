# `RegisterCallback`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · Janelas da GUI e automação de janelas

## Descrição

Creates a machine-code callback address for a script function.

## Sintaxe

```ahk
Addr := RegisterCallback(Function, Options, ParamCount)
```

## Parâmetros

- **Function** — The function.
- **Options** — CDecl/Fast/Indirect.
- **ParamCount** — Params.

## Valor de retorno

An address; this tree refuses at run (it does not manufacture machine code).

## Exemplo

```ahk
cb := RegisterCallback("MyFunc")
```

## Relacionados

[`DllCall`](DllCall.md), [`CallbackCreate`](CallbackCreate.md)
