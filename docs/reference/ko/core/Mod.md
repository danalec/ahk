# `Mod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

The remainder of a division. The sign of the remainder is the sign of the dividend, and the answer is a whole number when both of the arguments are.

## 구문

```ahk
R := Mod(Dividend, Divisor)
```

## 매개변수

- **Dividend** — The number.
- **Divisor** — Non-zero.

## 반환 값

the remainder

## 오류

a divisor of zero fails with "Divide by zero."; a value that is not a number is refused with "Invalid parameter(s).".

## 설명

A zero divisor throws ZeroDivisionError.

## 예제

```ahk
Mod(7, 3)
```

## 관련

[`Abs`](Abs.md), [`Round`](Round.md), [`//`](__.md)
