# `VarSetStrCapacity`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · function · Chamadas a funções nativas (DllCall)

## Descrição

Sets the capacity of a variable's string buffer (or frees it).

## Sintaxe

```ahk
VarSetStrCapacity Var, Capacity
```

## Parâmetros

- **Var** — The variable.
- **Capacity** — Chars; -1 frees.

## Valor de retorno

Nothing.

## Exemplo

```ahk
VarSetStrCapacity s, 1024
```

## Relacionados

[`StrLen`](StrLen.md), [`Buffer`](Buffer.md)
