# `Mod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · 言語の中核: テキスト、数値、正規表現

## 説明

The remainder of a division. The sign of the remainder is the sign of the dividend, and the answer is a whole number when both of the arguments are.

## 構文

```ahk
R := Mod(Dividend, Divisor)
```

## パラメータ

- **Dividend** — The number.
- **Divisor** — Non-zero.

## 戻り値

the remainder

## エラー

a divisor of zero fails with "Divide by zero."; a value that is not a number is refused with "Invalid parameter(s).".

## 備考

A zero divisor throws ZeroDivisionError.

## 例

```ahk
Mod(7, 3)
```

## 関連

[`Abs`](Abs.md), [`Round`](Round.md), [`//`](__.md)
