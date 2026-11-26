# `Chr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · Ядро языка: текст, математика и шаблоны

## Описание

The character with a code point.

## Синтаксис

```ahk
S := Chr(Number)
```

## Параметры

- **Number** — The code point.

## Возвращаемое значение

the text of that character

## Ошибки

a code outside 0 to 0x10FFFF is an invalid parameter; a surrogate code is a text of its own.

## Пример

```ahk
Chr(65)
```

## См. также

[`Ord`](Ord.md), [`Asc`](Asc.md)
