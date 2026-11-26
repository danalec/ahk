# `Mod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · 語言核心：文字、數學與模式

## 描述

The remainder of a division. The sign of the remainder is the sign of the dividend, and the answer is a whole number when both of the arguments are.

## 語法

```ahk
R := Mod(Dividend, Divisor)
```

## 參數

- **Dividend** — The number.
- **Divisor** — Non-zero.

## 傳回值

the remainder

## 錯誤

a divisor of zero fails with "Divide by zero."; a value that is not a number is refused with "Invalid parameter(s).".

## 備註

A zero divisor throws ZeroDivisionError.

## 範例

```ahk
Mod(7, 3)
```

## 相關

[`Abs`](Abs.md), [`Round`](Round.md), [`//`](__.md)
