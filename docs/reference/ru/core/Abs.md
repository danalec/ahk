# `Abs`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · Ядро языка: текст, математика и шаблоны

## Описание

The distance from zero. An integer stays an integer.

## Синтаксис

```ahk
N := Abs(Number)
```

## Параметры

- **Number** — Integer or float.

## Возвращаемое значение

the number without its sign

## Ошибки

a value that is not a number is refused.

## Замечания

Scalar; no SIMD gain (docs/simd.md).

## Пример

```ahk
Abs(-3)
```

## См. также

[`Mod`](Mod.md), [`Round`](Round.md), [`Min`](Min.md), [`Max`](Max.md)
