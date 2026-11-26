# `StringGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Операции с файлами и каталогами

## Описание

The position of a substring (v1, 0-based).

## Синтаксис

```ahk
StringGetPos Out, Haystack, Needle
```

## Параметры

3..255.

## Возвращаемое значение

Nothing; 0-based pos or -1.

## Пример

```ahk
StringGetPos p, s, "x"
```

## См. также

[`InStr`](InStr.md)
