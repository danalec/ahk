# `IsNumber`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · function · Dilin çekirdeği: metin, matematik ve kalıplar

## Açıklama

Whether a value is a number: an integer, a float, or a text that holds one. A hexadecimal text is a number too.

## Sözdizimi

```ahk
B := IsNumber(Value)
```

## Parametreler

- **Value** — Any value.

## Dönüş değeri

1 when it is a number, 0 when it is not

## Hatalar

a second parameter is refused with "the code page of the text is not carried yet.".

## Örnek

```ahk
IsNumber("42")
```

## İlgili

[`IsInteger`](IsInteger.md), [`IsFloat`](IsFloat.md)
