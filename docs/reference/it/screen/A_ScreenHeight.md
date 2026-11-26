# `A_ScreenHeight`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · variable · Cattura dello schermo e funzioni sui pixel

## Descrizione

The height of the screen of the desktop, in pixels.

## Sintassi

```ahk
A_ScreenHeight 
```

## Parametri

nothing; it is a variable, not a call

## Valore di ritorno

the height, as a number

## Errori

the read fails with the reason of the desktop when the screen cannot be measured; the write is refused.

## Esempio

```ahk
FileAppend A_ScreenHeight, "*"
```
