# `VarSetStrCapacity`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · function · Chiamate a funzioni native (DllCall)

## Descrizione

Sets the capacity of a variable's string buffer (or frees it).

## Sintassi

```ahk
VarSetStrCapacity Var, Capacity
```

## Parametri

- **Var** — The variable.
- **Capacity** — Chars; -1 frees.

## Valore di ritorno

Nothing.

## Esempio

```ahk
VarSetStrCapacity s, 1024
```

## Correlati

[`StrLen`](StrLen.md), [`Buffer`](Buffer.md)
