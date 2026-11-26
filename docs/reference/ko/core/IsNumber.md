# `IsNumber`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · function · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

Whether a value is a number: an integer, a float, or a text that holds one. A hexadecimal text is a number too.

## 구문

```ahk
B := IsNumber(Value)
```

## 매개변수

- **Value** — Any value.

## 반환 값

1 when it is a number, 0 when it is not

## 오류

a second parameter is refused with "the code page of the text is not carried yet.".

## 예제

```ahk
IsNumber("42")
```

## 관련

[`IsInteger`](IsInteger.md), [`IsFloat`](IsFloat.md)
