# `WinGetStyle`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · command · Ventanas de la GUI y automatización de ventanas

## Descripción

The style of a window: the number of the bits of Windows, the `WS_*` ones that say how the window is decorated and what it may do. The style of the window of the script is 80674816.

## Sintaxis

```ahk
WinGetStyle …
```

## Parámetros

WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## Valor de retorno

the style, as a number

## Errores

a window that is not there refuses with 'Target window not found.', and the title that was asked about is in the `Extra`; the text of the window and the two exclusions are not carried yet.

## Ejemplo

```ahk
FileAppend WinGetStyle("ZZZ_NO_WINDOW_ZZZ"), "*"
```
