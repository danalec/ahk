# `RegisterCallback`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · function · GUI-Fenster und Fensterautomatisierung

## Beschreibung

Creates a machine-code callback address for a script function.

## Syntax

```ahk
Addr := RegisterCallback(Function, Options, ParamCount)
```

## Parameter

- **Function** — The function.
- **Options** — CDecl/Fast/Indirect.
- **ParamCount** — Params.

## Rückgabewert

An address; this tree refuses at run (it does not manufacture machine code).

## Beispiel

```ahk
cb := RegisterCallback("MyFunc")
```

## Verwandt

[`DllCall`](DllCall.md), [`CallbackCreate`](CallbackCreate.md)
