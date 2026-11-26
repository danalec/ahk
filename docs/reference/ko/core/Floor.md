# `Floor`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

The whole number below a number.

## 구문

```ahk
N := Floor(Number)
```

## 매개변수

- **Number** — The value.

## 반환 값

the whole number, a float when the number was not whole

## 오류

a value that is not a number is refused.

## 예제

```ahk
Floor(1.8)
```

## 관련

[`Ceil`](Ceil.md), [`Round`](Round.md)
