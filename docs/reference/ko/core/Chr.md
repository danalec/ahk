# `Chr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

The character with a code point.

## 구문

```ahk
S := Chr(Number)
```

## 매개변수

- **Number** — The code point.

## 반환 값

the text of that character

## 오류

a code outside 0 to 0x10FFFF is an invalid parameter; a surrogate code is a text of its own.

## 예제

```ahk
Chr(65)
```

## 관련

[`Ord`](Ord.md), [`Asc`](Asc.md)
