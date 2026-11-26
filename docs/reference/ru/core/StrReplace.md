# `StrReplace`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · Ядро языка: текст, математика и шаблоны

## Описание

The text with one part of it swapped for another. The case sense says whether the letters are taken as they are, and it is off when the caller does not say.

## Синтаксис

```ahk
New := StrReplace(Haystack, Search, Replace, CaseSense, OutputVarCount, Limit)
```

## Параметры

- **Haystack** — The text.
- **Search** — What to find.
- **Replace** — What to put (default empty).
- **CaseSense** — On/Off.
- **OutputVarCount** — Receives the number of replacements.
- **Limit** — Max replacements (-1 all).

## Возвращаемое значение

the text with the replacements

## Ошибки

the count must be a variable reference; a needle that is empty changes nothing and the text comes back as it is; the logical case sense is an invalid parameter.

## Замечания

An empty Search is refused (Parameter #2 invalid).

## Пример

```ahk
StrReplace("a-b-c", "-", "+")
```

## См. также

[`InStr`](InStr.md), [`RegExReplace`](RegExReplace.md), [`StrSplit`](StrSplit.md)
