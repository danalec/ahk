# `StrLen`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · Dilin çekirdeği: metin, matematik ve kalıplar

## Açıklama

Counts the characters of a text in the code units of UTF-16, so a character outside the basic plane counts as two.

## Sözdizimi

```ahk
Count := StrLen(String)
```

## Parametreler

- **String** — The text.

## Dönüş değeri

the number of code units

## Hatalar

none; a value that is not text is read as its text.

## Notlar

Astral characters count as one each on this tree.

## Örnek

```ahk
StrLen("hello")
```

## İlgili

[`SubStr`](SubStr.md), [`InStr`](InStr.md)
