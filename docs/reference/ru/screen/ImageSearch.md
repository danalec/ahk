# `ImageSearch`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Захват экрана и пиксельные функции

## Описание

Looks for a picture on the screen. The region is captured once and the picture is looked for in it, with the variation and the transparent colour.

## Синтаксис

```ahk
ImageSearch …
```

## Параметры

&X, &Y, X1, Y1, X2, Y2, Picture [, Options]

## Возвращаемое значение

one when the picture is there, zero when it is not

## Ошибки

a picture that cannot be loaded is refused with 'Parameter #7 of ImageSearch is invalid.'; on a host without `grim` the search fails with the reason of `grim`.

## Пример

```ahk
ImageSearch &x, &y, 0, 0, 800, 600, "button.png"
```
