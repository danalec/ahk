# `Round`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

The number to the number of decimals the caller asks for. With a positive number of decimals the answer is a text of exactly that many digits after the point, zeros kept; with the number omitted, zero or negative it is a whole number.

## 구문

```ahk
N := Round(Number, N)
```

## 매개변수

- **Number** — The value.
- **N** — Decimals; negative rounds tens/hundreds.

## 반환 값

a text with the decimals asked for, or a whole number

## 오류

a text that does not hold a number is refused; more than 100000 decimals is an invalid parameter.

## 설명

Half-away-from-zero.

## 예제

```ahk
Round(2.567, 2)
```

## 관련

[`Floor`](Floor.md), [`Ceil`](Ceil.md), [`Mod`](Mod.md)
