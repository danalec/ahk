# `VarSetStrCapacity`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · function · Aufrufe nativer Funktionen (DllCall)

## Beschreibung

Sets the capacity of a variable's string buffer (or frees it).

## Syntax

```ahk
VarSetStrCapacity Var, Capacity
```

## Parameter

- **Var** — The variable.
- **Capacity** — Chars; -1 frees.

## Rückgabewert

Nothing.

## Beispiel

```ahk
VarSetStrCapacity s, 1024
```

## Verwandt

[`StrLen`](StrLen.md), [`Buffer`](Buffer.md)
