# `WinGetTitle`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Ventanas de la GUI y automatización de ventanas

## Descripción

The title of the active window, or of the window that matches the text that was given.

## Sintaxis

```ahk
T := WinGetTitle(WinTitle, WinText)
```

## Parámetros

- **WinTitle** — The criteria; omitted = last found.

## Valor de retorno

the title of the window

## Errores

a window that was named and is not there answers 'Target window not found.'; the text of the window and the exclusions are not carried yet.

## Ejemplo

```ahk
title := WinGetTitle()
```

## Relacionados

[`WinGetText`](WinGetText.md), [`WinGetPos`](WinGetPos.md)
