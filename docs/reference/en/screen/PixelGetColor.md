# `PixelGetColor`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · Screen capture and pixel functions

## Description

The colour of one pixel, read from the frame the compositor draws. The box captures a 1x1 region with `grim` and reads the one pixel.

## Syntax

```ahk
C := PixelGetColor(X, Y, Mode)
```

## Parameters

- **X/Y** — Screen coords.
- **Mode** — Alt/Slow/RGB.

## Return value

the colour, as `#rrggbb`

## Errors

a colour written as a number is refused with 'is not a colour: write it as #rrggbb'; on a host without `grim` the read fails with the reason of `grim`.

## Example

```ahk
colour := PixelGetColor(10, 20)
```

## Related

[`PixelSearch`](PixelSearch.md), [`ImageSearch`](ImageSearch.md)
