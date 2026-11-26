# `Trim`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · Ядро языка: текст, математика и шаблоны

## Описание

The text without the characters at its ends.

## Синтаксис

```ahk
Trimmed := Trim(String, OmitChars)
```

## Параметры

- **String** — The text.
- **OmitChars** — Default space+tab.

## Возвращаемое значение

the text without the leading and the trailing characters of the set; the set is the space and the tab when the caller does not name one

## Ошибки

none.

## Замечания

LTrim/RTrim trim one end.

## Пример

```ahk
Trim("  hello  ")
```

## См. также

[`LTrim`](LTrim.md), [`RTrim`](RTrim.md)
