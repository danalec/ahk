# `LoopRegKey`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · variable · APIs do registro

## Descrição

The name of the current subkey in a Loop Reg.

## Sintaxe

```ahk
S := LoopRegKey
```

## Parâmetros

?.

## Valor de retorno

The subkey name.

## Exemplo

```ahk
Loop Reg, "HKCU\Software"
    s := LoopRegKey
```

## Relacionados

[`LoopRegName`](LoopRegName.md), [`LoopRegType`](LoopRegType.md)
