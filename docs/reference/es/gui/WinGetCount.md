# `WinGetCount`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · command · Ventanas de la GUI y automatización de ventanas

## Descripción

How many windows match the title that was given.

## Sintaxis

```ahk
WinGetCount …
```

## Parámetros

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## Valor de retorno

the number of the windows, a number

## Errores

the text of the window and the exclusions are not carried yet; a question the compositor cannot answer fails with its reason.

## Ejemplo

```ahk
count := WinGetCount("Untitled")
```
