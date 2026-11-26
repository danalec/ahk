# `LoopRegKey`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · variable · Interfejsy API rejestru

## Opis

The name of the current subkey in a Loop Reg.

## Składnia

```ahk
S := LoopRegKey
```

## Parametry

?.

## Wartość zwracana

The subkey name.

## Przykład

```ahk
Loop Reg, "HKCU\Software"
    s := LoopRegKey
```

## Powiązane

[`LoopRegName`](LoopRegName.md), [`LoopRegType`](LoopRegType.md)
