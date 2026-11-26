# `StrLen`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

Counts the characters of a text in the code units of UTF-16, so a character outside the basic plane counts as two.

## 구문

```ahk
Count := StrLen(String)
```

## 매개변수

- **String** — The text.

## 반환 값

the number of code units

## 오류

none; a value that is not text is read as its text.

## 설명

Astral characters count as one each on this tree.

## 예제

```ahk
StrLen("hello")
```

## 관련

[`SubStr`](SubStr.md), [`InStr`](InStr.md)
