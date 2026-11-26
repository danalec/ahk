# `VarSetStrCapacity`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · function · Wywołania funkcji natywnych (DllCall)

## Opis

Sets the capacity of a variable's string buffer (or frees it).

## Składnia

```ahk
VarSetStrCapacity Var, Capacity
```

## Parametry

- **Var** — The variable.
- **Capacity** — Chars; -1 frees.

## Wartość zwracana

Nothing.

## Przykład

```ahk
VarSetStrCapacity s, 1024
```

## Powiązane

[`StrLen`](StrLen.md), [`Buffer`](Buffer.md)
