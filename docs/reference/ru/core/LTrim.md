# `LTrim`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · function · Ядро языка: текст, математика и шаблоны

## Описание

The text without the characters at its left end.

## Синтаксис

```ahk
S := LTrim(String, OmitChars)
```

## Параметры

- **String** — The text.
- **OmitChars** — Default space+tab.

## Возвращаемое значение

the text without the leading characters of the set; the set is the space and the tab when the caller does not name one

## Ошибки

none.

## Пример

```ahk
LTrim("  hello  ")
```

## См. также

[`Trim`](Trim.md), [`RTrim`](RTrim.md)
