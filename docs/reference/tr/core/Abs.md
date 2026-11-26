# `Abs`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · Dilin çekirdeği: metin, matematik ve kalıplar

## Açıklama

The distance from zero. An integer stays an integer.

## Sözdizimi

```ahk
N := Abs(Number)
```

## Parametreler

- **Number** — Integer or float.

## Dönüş değeri

the number without its sign

## Hatalar

a value that is not a number is refused.

## Notlar

Scalar; no SIMD gain (docs/simd.md).

## Örnek

```ahk
Abs(-3)
```

## İlgili

[`Mod`](Mod.md), [`Round`](Round.md), [`Min`](Min.md), [`Max`](Max.md)
