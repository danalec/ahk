# `Abs`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

The distance from zero. An integer stays an integer.

## 구문

```ahk
N := Abs(Number)
```

## 매개변수

- **Number** — Integer or float.

## 반환 값

the number without its sign

## 오류

a value that is not a number is refused.

## 설명

Scalar; no SIMD gain (docs/simd.md).

## 예제

```ahk
Abs(-3)
```

## 관련

[`Mod`](Mod.md), [`Round`](Round.md), [`Min`](Min.md), [`Max`](Max.md)
