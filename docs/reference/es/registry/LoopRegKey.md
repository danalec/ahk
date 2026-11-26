# `LoopRegKey`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · variable · API del registro

## Descripción

The name of the current subkey in a Loop Reg.

## Sintaxis

```ahk
S := LoopRegKey
```

## Parámetros

?.

## Valor de retorno

The subkey name.

## Ejemplo

```ahk
Loop Reg, "HKCU\Software"
    s := LoopRegKey
```

## Relacionados

[`LoopRegName`](LoopRegName.md), [`LoopRegType`](LoopRegType.md)
