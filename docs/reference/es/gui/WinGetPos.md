# `WinGetPos`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Ventanas de la GUI y automatización de ventanas

## Descripción

Where the window is and how big it is, as the compositor reports it.

## Sintaxis

```ahk
WinGetPos X, Y, W, H, WinTitle
```

## Parámetros

- **Output*** — Variables.
- **WinTitle** — The criteria.

## Valor de retorno

nothing; the place and the size are written into the variables of the caller

## Errores

the text of the window and the exclusions are not carried yet; a window that is not there answers 'Target window not found.'

## Ejemplo

```ahk
WinGetPos "Untitled", , &x, &y, &w, &h
```

## Relacionados

[`WinMove`](WinMove.md), [`WinGetTitle`](WinGetTitle.md)
