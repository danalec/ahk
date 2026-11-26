# `SubStr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · Dilin çekirdeği: metin, matematik ve kalıplar

## Açıklama

The part of a text between two positions. The positions count from one, and a negative position counts from the end: -1 is the last character.

## Sözdizimi

```ahk
Part := SubStr(String, Start, Length)
```

## Parametreler

- **String** — The text.
- **Start** — 1-based; negative counts from the end.
- **Length** — Omitted = to the end.

## Dönüş değeri

the text of that part; a negative length leaves that many characters out at the end

## Hatalar

none; the text is cut in the code units of UTF-16, so the second unit of a surrogate pair is a text of its own.

## Notlar

Counts characters, not bytes.

## Örnek

```ahk
SubStr("hello", 2, 3)
```

## İlgili

[`StrLen`](StrLen.md), [`InStr`](InStr.md), [`StrSplit`](StrSplit.md)
