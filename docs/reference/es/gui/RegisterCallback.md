# `RegisterCallback`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · Ventanas de la GUI y automatización de ventanas

## Descripción

Creates a machine-code callback address for a script function.

## Sintaxis

```ahk
Addr := RegisterCallback(Function, Options, ParamCount)
```

## Parámetros

- **Function** — The function.
- **Options** — CDecl/Fast/Indirect.
- **ParamCount** — Params.

## Valor de retorno

An address; this tree refuses at run (it does not manufacture machine code).

## Ejemplo

```ahk
cb := RegisterCallback("MyFunc")
```

## Relacionados

[`DllCall`](DllCall.md), [`CallbackCreate`](CallbackCreate.md)
