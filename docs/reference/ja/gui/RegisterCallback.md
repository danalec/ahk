# `RegisterCallback`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · GUIウィンドウとウィンドウの自動化

## 説明

Creates a machine-code callback address for a script function.

## 構文

```ahk
Addr := RegisterCallback(Function, Options, ParamCount)
```

## パラメータ

- **Function** — The function.
- **Options** — CDecl/Fast/Indirect.
- **ParamCount** — Params.

## 戻り値

An address; this tree refuses at run (it does not manufacture machine code).

## 例

```ahk
cb := RegisterCallback("MyFunc")
```

## 関連

[`DllCall`](DllCall.md), [`CallbackCreate`](CallbackCreate.md)
