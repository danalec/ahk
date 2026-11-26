# `ImageSearch`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Przechwytywanie ekranu i funkcje pikseli

## Opis

Looks for a picture on the screen. The region is captured once and the picture is looked for in it, with the variation and the transparent colour.

## Składnia

```ahk
ImageSearch …
```

## Parametry

&X, &Y, X1, Y1, X2, Y2, Picture [, Options]

## Wartość zwracana

one when the picture is there, zero when it is not

## Błędy

a picture that cannot be loaded is refused with 'Parameter #7 of ImageSearch is invalid.'; on a host without `grim` the search fails with the reason of `grim`.

## Przykład

```ahk
ImageSearch &x, &y, 0, 0, 800, 600, "button.png"
```
