# `MouseClick`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Hotkeys, hotstrings y entrada

## Descripción

Clicks with the pointer of the backend. The button is the first parameter, the way the function of v2 takes it, and a button that is not there is the left one.

## Sintaxis

```ahk
MouseClick …
```

## Parámetros

Button, X, Y [Count] [Options]

## Valor de retorno

nothing

## Errores

the wheel, the fourth and the fifth buttons are refused as not carried yet.

## Ejemplo

```ahk
MouseClick "Right", 100, 200
```
