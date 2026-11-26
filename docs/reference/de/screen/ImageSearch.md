# `ImageSearch`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · Bildschirmaufnahme und Pixel-Funktionen

## Beschreibung

Looks for a picture on the screen. The region is captured once and the picture is looked for in it, with the variation and the transparent colour.

## Syntax

```ahk
ImageSearch …
```

## Parameter

&X, &Y, X1, Y1, X2, Y2, Picture [, Options]

## Rückgabewert

one when the picture is there, zero when it is not

## Fehler

a picture that cannot be loaded is refused with 'Parameter #7 of ImageSearch is invalid.'; on a host without `grim` the search fails with the reason of `grim`.

## Beispiel

```ahk
ImageSearch &x, &y, 0, 0, 800, 600, "button.png"
```
