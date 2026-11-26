# `SendText`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · command · Hotkeys, hotstrings y entrada

## Descripción

Types the text as it stands: the braces of v2 are not read here, and the text goes to the keyboard one character at a time.

## Sintaxis

```ahk
SendText …
```

## Parámetros

Text

## Valor de retorno

nothing

## Errores

the keyboard of the backend is a Linux uinput device, so on a host that is not Linux the text is not typed.

## Ejemplo

```ahk
SendText "a literal {braced} text"
```
