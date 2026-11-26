# `ControlFindItem`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · command · Ventanas de la GUI y automatización de ventanas

## Descripción

Finds an item by text.

## Sintaxis

```ahk
N := ControlFindItem(Text, Control, WinTitle)
```

## Parámetros

2..6.

## Valor de retorno

The index or 0.

## Ejemplo

```ahk
ControlFindItem("x", "ListBox1")
```

## Relacionados

[`ControlSend`](ControlSend.md), [`Gui`](Gui.md)
