# `ImageSearch`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Capture d'écran et fonctions de pixel

## Description

Looks for a picture on the screen. The region is captured once and the picture is looked for in it, with the variation and the transparent colour.

## Syntaxe

```ahk
ImageSearch …
```

## Paramètres

&X, &Y, X1, Y1, X2, Y2, Picture [, Options]

## Valeur de retour

one when the picture is there, zero when it is not

## Erreurs

a picture that cannot be loaded is refused with 'Parameter #7 of ImageSearch is invalid.'; on a host without `grim` the search fails with the reason of `grim`.

## Exemple

```ahk
ImageSearch &x, &y, 0, 0, 800, 600, "button.png"
```
