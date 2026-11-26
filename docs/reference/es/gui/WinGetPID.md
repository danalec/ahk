# `WinGetPID`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · command · Ventanas de la GUI y automatización de ventanas

## Descripción

Answers the process of a window, through the accessibility bus of the session: the registry of the bus names the applications, a window is a child of one, and the bus answers its PID.

## Sintaxis

```ahk
WinGetPID …
```

## Parámetros

WinTitle

## Valor de retorno

the pid of the process, as an integer

## Errores

"Target window not found." when no window has that title; an empty title is refused with "the window is found by its title, and an empty title is not carried yet."; the text of the window and the exclusions are refused with "the text of the window and the exclusions are not carried yet."

## Ejemplo

```ahk
pid := WinGetPID("Calculator")
```
