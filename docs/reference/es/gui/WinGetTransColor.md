# `WinGetTransColor`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · command · Ventanas de la GUI y automatización de ventanas

## Descripción

The colour a window lets through: the colour key of a layered window, the one that is not drawn. The colour is written as "0xRRGGBB", in the order of the bits of the red, the green and the blue. A window that is not layered answers the empty text.

## Sintaxis

```ahk
WinGetTransColor …
```

## Parámetros

WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## Valor de retorno

the colour as "0xRRGGBB", or the empty text when the window is not layered

## Errores

a window that is not there refuses with 'Target window not found.', and the title that was asked about is in the `Extra`; the text of the window and the two exclusions are not carried yet.

## Ejemplo

```ahk
FileAppend WinGetTransColor("ZZZ_NO_WINDOW_ZZZ"), "*"
```
