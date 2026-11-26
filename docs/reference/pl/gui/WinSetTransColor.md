# `WinSetTransColor`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · command · Okna GUI i automatyzacja okien

## Opis

Writes the colour a window lets through. The colour is the name of one of the seventeen colours of HTML, a hexadecimal number, or the word "Off", which takes the colour key away.

## Składnia

```ahk
WinSetTransColor …
```

## Parametry

Color, WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## Wartość zwracana

the empty text

## Błędy

a text that is not a colour refuses with 'Parameter #1 of WinSetTransColor is invalid.', and the text is in the `Extra`; a window that is not there refuses with 'Target window not found.'.

## Przykład

```ahk
WinSetTransColor("Red", "ZZZ_NO_WINDOW_ZZZ")
```
