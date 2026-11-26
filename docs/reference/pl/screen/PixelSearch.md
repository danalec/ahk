# `PixelSearch`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Przechwytywanie ekranu i funkcje pikseli

## Opis

Where a colour is on the screen. The region is captured once and walked for the first pixel of that colour. It is the check that works whatever the desktop is.

## Składnia

```ahk
PixelSearch OutputX, OutputY, X1, Y1, X2, Y2, Color, Variation
```

## Parametry

- **Output*** — Receive the found coords.
- **Region** — The rectangle.
- **Color** — 0xRRGGBB.
- **Variation** — 0..255 per channel.

## Wartość zwracana

one when the colour is there, zero when it is not

## Błędy

the two places are written into the variables of the caller, and they are made blank when the colour is not there; on a host without `grim` the search fails with the reason of `grim`.

## Przykład

```ahk
PixelSearch &x, &y, 0, 0, 100, 100, "#ff0000"
```

## Powiązane

[`PixelGetColor`](PixelGetColor.md), [`ImageSearch`](ImageSearch.md)
