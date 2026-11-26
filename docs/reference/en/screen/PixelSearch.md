# `PixelSearch`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · Screen capture and pixel functions

## Description

Where a colour is on the screen. The region is captured once and walked for the first pixel of that colour. It is the check that works whatever the desktop is.

## Syntax

```ahk
PixelSearch OutputX, OutputY, X1, Y1, X2, Y2, Color, Variation
```

## Parameters

- **Output*** — Receive the found coords.
- **Region** — The rectangle.
- **Color** — 0xRRGGBB.
- **Variation** — 0..255 per channel.

## Return value

one when the colour is there, zero when it is not

## Errors

the two places are written into the variables of the caller, and they are made blank when the colour is not there; on a host without `grim` the search fails with the reason of `grim`.

## Example

```ahk
PixelSearch &x, &y, 0, 0, 100, 100, "#ff0000"
```

## Related

[`PixelGetColor`](PixelGetColor.md), [`ImageSearch`](ImageSearch.md)
