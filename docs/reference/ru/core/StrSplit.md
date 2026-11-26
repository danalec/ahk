# `StrSplit`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · Ядро языка: текст, математика и шаблоны

## Описание

The pieces of a text, cut at the delimiters. A delimiter is a literal: a text is one delimiter, an array of texts is a set of them, and the first one of them that is there wins.

## Синтаксис

```ahk
Array := StrSplit(String, Delimiters, OmitChars, MaxParts)
```

## Параметры

- **String** — The text.
- **Delimiters** — A string of delimiter chars or an array.
- **OmitChars** — Chars to trim from each part.
- **MaxParts** — Limit the number of parts.

## Возвращаемое значение

an array of the pieces; the characters to omit are cut from the beginning and the end of every piece

## Ошибки

the delimiters must be a text or an array of texts; an empty array is refused with "Parameter #2 of StrSplit is invalid.".

## Замечания

Empty delimiters split into characters.

## Пример

```ahk
StrSplit("a,b,c", ",")
```

## См. также

[`StrReplace`](StrReplace.md), [`Array`](Array.md), [`LoopParse`](LoopParse.md)
