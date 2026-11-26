# `LoopRegKey`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · variable · Le API del registro

## Descrizione

The name of the current subkey in a Loop Reg.

## Sintassi

```ahk
S := LoopRegKey
```

## Parametri

?.

## Valore di ritorno

The subkey name.

## Esempio

```ahk
Loop Reg, "HKCU\Software"
    s := LoopRegKey
```

## Correlati

[`LoopRegName`](LoopRegName.md), [`LoopRegType`](LoopRegType.md)
