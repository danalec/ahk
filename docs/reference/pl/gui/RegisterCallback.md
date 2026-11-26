# `RegisterCallback`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · function · Okna GUI i automatyzacja okien

## Opis

Creates a machine-code callback address for a script function.

## Składnia

```ahk
Addr := RegisterCallback(Function, Options, ParamCount)
```

## Parametry

- **Function** — The function.
- **Options** — CDecl/Fast/Indirect.
- **ParamCount** — Params.

## Wartość zwracana

An address; this tree refuses at run (it does not manufacture machine code).

## Przykład

```ahk
cb := RegisterCallback("MyFunc")
```

## Powiązane

[`DllCall`](DllCall.md), [`CallbackCreate`](CallbackCreate.md)
