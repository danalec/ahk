# `SendMessage`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · command · Ventanas de la GUI y automatización de ventanas

## Descripción

Sends a message to a window and waits.

## Sintaxis

```ahk
R := SendMessage(Msg, wParam, lParam, Control, WinTitle, Timeout)
```

## Parámetros

1..9.

## Valor de retorno

The result.

## Ejemplo

```ahk
SendMessage(0x100, 0, 0)
```

## Relacionados

[`Menu`](Menu.md), [`Gui`](Gui.md), [`OnMessage`](OnMessage.md)
