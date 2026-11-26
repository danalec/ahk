# `RegisterCallback`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · GUI 창과 창 자동화

## 설명

Creates a machine-code callback address for a script function.

## 구문

```ahk
Addr := RegisterCallback(Function, Options, ParamCount)
```

## 매개변수

- **Function** — The function.
- **Options** — CDecl/Fast/Indirect.
- **ParamCount** — Params.

## 반환 값

An address; this tree refuses at run (it does not manufacture machine code).

## 예제

```ahk
cb := RegisterCallback("MyFunc")
```

## 관련

[`DllCall`](DllCall.md), [`CallbackCreate`](CallbackCreate.md)
