# `LoopRegTimeModified`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · variable · Le API del registro

## Descrizione

The last-write time of the current registry key.

## Sintassi

```ahk
S := LoopRegTimeModified
```

## Parametri

?.

## Valore di ritorno

A YYYYMMDDHH24MISS timestamp.

## Esempio

```ahk
t := LoopRegTimeModified
```

## Correlati

[`LoopRegKey`](LoopRegKey.md), [`FormatTime`](FormatTime.md)
