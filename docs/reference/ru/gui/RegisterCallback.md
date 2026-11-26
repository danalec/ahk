# `RegisterCallback`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · Окна интерфейса и автоматизация окон

## Описание

Creates a machine-code callback address for a script function.

## Синтаксис

```ahk
Addr := RegisterCallback(Function, Options, ParamCount)
```

## Параметры

- **Function** — The function.
- **Options** — CDecl/Fast/Indirect.
- **ParamCount** — Params.

## Возвращаемое значение

An address; this tree refuses at run (it does not manufacture machine code).

## Пример

```ahk
cb := RegisterCallback("MyFunc")
```

## См. также

[`DllCall`](DllCall.md), [`CallbackCreate`](CallbackCreate.md)
