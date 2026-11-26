# `Ord`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · Ядро языка: текст, математика и шаблоны

## Описание

The code point of the first character of a text.

## Синтаксис

```ahk
N := Ord(String)
```

## Параметры

- **String** — The text.

## Возвращаемое значение

the code point, or zero when the text has no character

## Ошибки

none; a lone surrogate answers with the code unit of it.

## Пример

```ahk
Ord("A")
```

## См. также

[`Chr`](Chr.md), [`Asc`](Asc.md)
