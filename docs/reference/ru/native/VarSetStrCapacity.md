# `VarSetStrCapacity`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · function · Вызовы нативных функций (DllCall)

## Описание

Sets the capacity of a variable's string buffer (or frees it).

## Синтаксис

```ahk
VarSetStrCapacity Var, Capacity
```

## Параметры

- **Var** — The variable.
- **Capacity** — Chars; -1 frees.

## Возвращаемое значение

Nothing.

## Пример

```ahk
VarSetStrCapacity s, 1024
```

## См. также

[`StrLen`](StrLen.md), [`Buffer`](Buffer.md)
