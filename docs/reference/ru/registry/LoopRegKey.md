# `LoopRegKey`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · variable · API реестра

## Описание

The name of the current subkey in a Loop Reg.

## Синтаксис

```ahk
S := LoopRegKey
```

## Параметры

?.

## Возвращаемое значение

The subkey name.

## Пример

```ahk
Loop Reg, "HKCU\Software"
    s := LoopRegKey
```

## См. также

[`LoopRegName`](LoopRegName.md), [`LoopRegType`](LoopRegType.md)
