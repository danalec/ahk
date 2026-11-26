# `Round`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · 言語の中核: テキスト、数値、正規表現

## 説明

The number to the number of decimals the caller asks for. With a positive number of decimals the answer is a text of exactly that many digits after the point, zeros kept; with the number omitted, zero or negative it is a whole number.

## 構文

```ahk
N := Round(Number, N)
```

## パラメータ

- **Number** — The value.
- **N** — Decimals; negative rounds tens/hundreds.

## 戻り値

a text with the decimals asked for, or a whole number

## エラー

a text that does not hold a number is refused; more than 100000 decimals is an invalid parameter.

## 備考

Half-away-from-zero.

## 例

```ahk
Round(2.567, 2)
```

## 関連

[`Floor`](Floor.md), [`Ceil`](Ceil.md), [`Mod`](Mod.md)
