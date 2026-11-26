# `Sort`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*KeySharp* · method · `Array` · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

The items of a text in order. The second parameter is the options, read letter by letter, and the third is a callback that compares two items.

## 구문

```ahk
S := Sort(String, Options)
```

## 매개변수

- **String** — The lines.
- **Options** — N (numeric), R (reverse), D delim, C case, etc.

## 반환 값

the text with the items in order

## 오류

the first parameter must be a text; an array is refused; a third parameter that cannot be called fails with "This value of type ... has no method named Call".

## 예제

```ahk
Sort("b`na`n", "N")
```

## 관련

[`StrSplit`](StrSplit.md), [`Array`](Array.md)
