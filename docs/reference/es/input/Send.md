# `Send`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Hotkeys, hotstrings y entrada

## Descripción

Types text and presses keys on the machine, through the uinput keyboard of the backend.

## Sintaxis

```ahk
Send Keys
```

## Parámetros

- **Keys** — The keystrokes, with {Enter}, ^c, !tab, {Blind}, etc.

## Valor de retorno

nothing

## Errores

the keyboard is a Linux uinput device, so on a host that is not Linux every key action fails; a character the US layout has no key for fails with 'the keyboard of this box has no key for'.

## Observaciones

SendMode chooses Input/Play/Event. On Linux it is the uinput/XTEST keyboard.

## Ejemplo

```ahk
Send "hello{Enter}"
```

## Relacionados

[`SendText`](SendText.md), [`ControlSend`](ControlSend.md), [`Click`](Click.md), [`SendMode`](SendMode.md)
