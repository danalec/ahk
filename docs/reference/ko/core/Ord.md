# `Ord`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

The code point of the first character of a text.

## 구문

```ahk
N := Ord(String)
```

## 매개변수

- **String** — The text.

## 반환 값

the code point, or zero when the text has no character

## 오류

none; a lone surrogate answers with the code unit of it.

## 예제

```ahk
Ord("A")
```

## 관련

[`Chr`](Chr.md), [`Asc`](Asc.md)
