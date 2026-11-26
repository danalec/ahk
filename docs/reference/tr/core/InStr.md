# `InStr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · Dilin çekirdeği: metin, matematik ve kalıplar

## Açıklama

Where a text is in another text. The position counts from one and is zero when the needle is not there at all.

## Sözdizimi

```ahk
Pos := InStr(Haystack, Needle, CaseSense, Start, Occurrence)
```

## Parametreler

- **Haystack** — The text to search.
- **Needle** — The text to find.
- **CaseSense** — On/Off/Locale.
- **Start** — 1-based, negative from end.
- **Occurrence** — Which hit.

## Dönüş değeri

the position of the match, or 0 when there is none

## Hatalar

a needle that is empty is an invalid parameter; the logical case sense is an invalid parameter; a start or an occurrence of zero is an invalid parameter.

## Notlar

The sensitive search uses the vectorized substring scan (see docs/simd.md).

## Örnek

```ahk
InStr("hello", "ll")
```

## İlgili

[`StrReplace`](StrReplace.md), [`SubStr`](SubStr.md), [`RegExMatch`](RegExMatch.md)
