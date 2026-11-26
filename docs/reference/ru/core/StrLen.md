# `StrLen`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · Ядро языка: текст, математика и шаблоны

## Описание

Counts the characters of a text in the code units of UTF-16, so a character outside the basic plane counts as two.

## Синтаксис

```ahk
Count := StrLen(String)
```

## Параметры

- **String** — The text.

## Возвращаемое значение

the number of code units

## Ошибки

none; a value that is not text is read as its text.

## Замечания

Astral characters count as one each on this tree.

## Пример

```ahk
StrLen("hello")
```

## См. также

[`SubStr`](SubStr.md), [`InStr`](InStr.md)
