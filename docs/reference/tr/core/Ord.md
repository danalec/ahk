# `Ord`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · Dilin çekirdeği: metin, matematik ve kalıplar

## Açıklama

The code point of the first character of a text.

## Sözdizimi

```ahk
N := Ord(String)
```

## Parametreler

- **String** — The text.

## Dönüş değeri

the code point, or zero when the text has no character

## Hatalar

none; a lone surrogate answers with the code unit of it.

## Örnek

```ahk
Ord("A")
```

## İlgili

[`Chr`](Chr.md), [`Asc`](Asc.md)
