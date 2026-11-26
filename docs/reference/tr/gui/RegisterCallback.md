# `RegisterCallback`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · GUI pencereleri ve pencere otomasyonu

## Açıklama

Creates a machine-code callback address for a script function.

## Sözdizimi

```ahk
Addr := RegisterCallback(Function, Options, ParamCount)
```

## Parametreler

- **Function** — The function.
- **Options** — CDecl/Fast/Indirect.
- **ParamCount** — Params.

## Dönüş değeri

An address; this tree refuses at run (it does not manufacture machine code).

## Örnek

```ahk
cb := RegisterCallback("MyFunc")
```

## İlgili

[`DllCall`](DllCall.md), [`CallbackCreate`](CallbackCreate.md)
