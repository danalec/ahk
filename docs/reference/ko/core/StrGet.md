# `StrGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

The text of a buffer, or the text at an address. The source is a buffer or a number; the length and the encoding come after it.

## 구문

```ahk
StrGet …
```

## 매개변수

Source, [Length], [Encoding]

## 반환 값

the text; an address that names no buffer and no text answers the empty text

## 오류

a text as the source is refused with "Parameter #1 invalid."; an encoding the dialect does not know is refused with "Invalid Encoding.".

## 예제

```ahk
StrGet(Buffer(16), 4, "UTF-8")
```
