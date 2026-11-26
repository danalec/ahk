# `Random`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

A number between the two the caller names. Without arguments it is a number between zero and one; with one it is a whole number below it; with two it is a whole number between them, both ends counted.

## 구문

```ahk
N := Random(Min, Max)
```

## 매개변수

- **Min/Max** — The range; omitted = float 0..1.
- **Seed** — With a single arg, reseeds.

## 반환 값

the number

## 오류

none.

## 예제

```ahk
Random(1, 6)
```

## 관련

[`Sort`](Sort.md)
