# `WinGetClass`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Ventanas de la GUI y automatización de ventanas

## Descripción

The class of the window that matches the title that was given.

## Sintaxis

```ahk
WinGetClass …
```

## Parámetros

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## Valor de retorno

the class of the window

## Errores

a title that matches no window answers 'Target window not found.'; the text of the window and the exclusions are not carried yet.

## Ejemplo

```ahk
class := WinGetClass("Untitled")
```
