# `Format`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · Ядро языка: текст, математика и шаблоны

## Описание

A pattern with the values in it. The pattern is the one of the reference dialect: {1} is the first argument, a placeholder with no number takes the next one, and {{ and }} are one brace of their own.

## Синтаксис

```ahk
Text := Format(FormatStr, Values...)
```

## Параметры

- **FormatStr** — With {1}, {2:0.00}, {:#x} placeholders.
- **Values** — The values to insert.

## Возвращаемое значение

the text of the pattern with the values in it

## Ошибки

a placeholder that names an argument that is not there, or whose spec is not one the dialect reads, is left exactly as it was written; a value of a kind the spec does not want is refused.

## Замечания

Supports width, precision, hex, and alignment.

## Пример

```ahk
Format("{1} + {2} = {3}", 1, 2, 3)
```

## См. также

[`StrReplace`](StrReplace.md), [`String`](String.md)
