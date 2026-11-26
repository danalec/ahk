# `Chr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · Dilin çekirdeği: metin, matematik ve kalıplar

## Açıklama

The character with a code point.

## Sözdizimi

```ahk
S := Chr(Number)
```

## Parametreler

- **Number** — The code point.

## Dönüş değeri

the text of that character

## Hatalar

a code outside 0 to 0x10FFFF is an invalid parameter; a surrogate code is a text of its own.

## Örnek

```ahk
Chr(65)
```

## İlgili

[`Ord`](Ord.md), [`Asc`](Asc.md)
