# `MouseGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Kısayollar, tetikleme dizeleri ve giriş

## Açıklama

Reads the place of the pointer this runtime drives, out of the kernel. It is the only pointer on the box whose place can be read at all: the physical mouse is a relative device.

## Sözdizimi

```ahk
MouseGetPos OutputX, OutputY, OutputWin, OutputControl, Flag
```

## Parametreler

- **Output*** — Variables that receive the answers.

## Dönüş değeri

nothing; the place is written into the variables of the caller

## Hatalar

when the kernel will not answer, the desktop answers the place the runtime last put the pointer at, and says so in a note.

## Örnek

```ahk
MouseGetPos &x, &y
```

## İlgili

[`MouseMove`](MouseMove.md), [`PixelGetColor`](PixelGetColor.md)
