# `LTrim`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · function · Dilin çekirdeği: metin, matematik ve kalıplar

## Açıklama

The text without the characters at its left end.

## Sözdizimi

```ahk
S := LTrim(String, OmitChars)
```

## Parametreler

- **String** — The text.
- **OmitChars** — Default space+tab.

## Dönüş değeri

the text without the leading characters of the set; the set is the space and the tab when the caller does not name one

## Hatalar

none.

## Örnek

```ahk
LTrim("  hello  ")
```

## İlgili

[`Trim`](Trim.md), [`RTrim`](RTrim.md)
