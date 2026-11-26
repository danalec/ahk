# `CoordMode`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Parser i wykonanie skryptu

## Opis

The way the coordinates are read. The five targets of v2 are the mouse, the pixels, the caret, the menu and the tooltip; the tree has no windows, so the modes of the caret, the menu and the tooltip are kept all the same and nothing uses them.

## Składnia

```ahk
CoordMode …
```

## Parametry

Target [, Relative]

## Wartość zwracana

the mode that was there before

## Błędy

a target or a relative that v2 does not know is refused as an invalid parameter.

## Przykład

```ahk
CoordMode "Mouse", "Screen"
```
