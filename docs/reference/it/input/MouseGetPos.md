# `MouseGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Tasti rapidi, stringhe di attivazione e input

## Descrizione

Reads the place of the pointer this runtime drives, out of the kernel. It is the only pointer on the box whose place can be read at all: the physical mouse is a relative device.

## Sintassi

```ahk
MouseGetPos OutputX, OutputY, OutputWin, OutputControl, Flag
```

## Parametri

- **Output*** — Variables that receive the answers.

## Valore di ritorno

nothing; the place is written into the variables of the caller

## Errori

when the kernel will not answer, the desktop answers the place the runtime last put the pointer at, and says so in a note.

## Esempio

```ahk
MouseGetPos &x, &y
```

## Correlati

[`MouseMove`](MouseMove.md), [`PixelGetColor`](PixelGetColor.md)
