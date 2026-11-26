# `ControlGetText`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Ventanas de la GUI y automatización de ventanas

## Descripción

Reads the text of a control.

## Sintaxis

```ahk
T := ControlGetText(Control, WinTitle)
```

## Parámetros

- **Control** — ClassNN.
- **WinTitle** — The window.

## Valor de retorno

The text.

## Ejemplo

```ahk
t := ControlGetText("Edit1", "ahk_class Notepad")
```

## Relacionados

[`ControlSetText`](ControlSetText.md), [`WinGetText`](WinGetText.md)
