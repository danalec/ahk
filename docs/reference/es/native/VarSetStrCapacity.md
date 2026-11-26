# `VarSetStrCapacity`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · function · Llamadas a funciones nativas (DllCall)

## Descripción

Sets the capacity of a variable's string buffer (or frees it).

## Sintaxis

```ahk
VarSetStrCapacity Var, Capacity
```

## Parámetros

- **Var** — The variable.
- **Capacity** — Chars; -1 frees.

## Valor de retorno

Nothing.

## Ejemplo

```ahk
VarSetStrCapacity s, 1024
```

## Relacionados

[`StrLen`](StrLen.md), [`Buffer`](Buffer.md)
