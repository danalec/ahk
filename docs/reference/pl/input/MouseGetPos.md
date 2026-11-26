# `MouseGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Skróty, ciągi wyzwalające i wprowadzanie

## Opis

Reads the place of the pointer this runtime drives, out of the kernel. It is the only pointer on the box whose place can be read at all: the physical mouse is a relative device.

## Składnia

```ahk
MouseGetPos OutputX, OutputY, OutputWin, OutputControl, Flag
```

## Parametry

- **Output*** — Variables that receive the answers.

## Wartość zwracana

nothing; the place is written into the variables of the caller

## Błędy

when the kernel will not answer, the desktop answers the place the runtime last put the pointer at, and says so in a note.

## Przykład

```ahk
MouseGetPos &x, &y
```

## Powiązane

[`MouseMove`](MouseMove.md), [`PixelGetColor`](PixelGetColor.md)
