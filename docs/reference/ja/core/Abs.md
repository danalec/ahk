# `Abs`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · 言語の中核: テキスト、数値、正規表現

## 説明

The distance from zero. An integer stays an integer.

## 構文

```ahk
N := Abs(Number)
```

## パラメータ

- **Number** — Integer or float.

## 戻り値

the number without its sign

## エラー

a value that is not a number is refused.

## 備考

Scalar; no SIMD gain (docs/simd.md).

## 例

```ahk
Abs(-3)
```

## 関連

[`Mod`](Mod.md), [`Round`](Round.md), [`Min`](Min.md), [`Max`](Max.md)
