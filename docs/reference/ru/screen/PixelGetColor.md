# `PixelGetColor`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Захват экрана и пиксельные функции

## Описание

The colour of one pixel, read from the frame the compositor draws. The box captures a 1x1 region with `grim` and reads the one pixel.

## Синтаксис

```ahk
C := PixelGetColor(X, Y, Mode)
```

## Параметры

- **X/Y** — Screen coords.
- **Mode** — Alt/Slow/RGB.

## Возвращаемое значение

the colour, as `#rrggbb`

## Ошибки

a colour written as a number is refused with 'is not a colour: write it as #rrggbb'; on a host without `grim` the read fails with the reason of `grim`.

## Пример

```ahk
colour := PixelGetColor(10, 20)
```

## См. также

[`PixelSearch`](PixelSearch.md), [`ImageSearch`](ImageSearch.md)
