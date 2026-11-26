# `RegisterCallback`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · Fenêtres d'interface et automatisation des fenêtres

## Description

Creates a machine-code callback address for a script function.

## Syntaxe

```ahk
Addr := RegisterCallback(Function, Options, ParamCount)
```

## Paramètres

- **Function** — The function.
- **Options** — CDecl/Fast/Indirect.
- **ParamCount** — Params.

## Valeur de retour

An address; this tree refuses at run (it does not manufacture machine code).

## Exemple

```ahk
cb := RegisterCallback("MyFunc")
```

## Voir aussi

[`DllCall`](DllCall.md), [`CallbackCreate`](CallbackCreate.md)
