# `A_ScreenWidth`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · variable · Bildschirmaufnahme und Pixel-Funktionen

## Beschreibung

The width of the screen of the desktop, in pixels.

## Syntax

```ahk
A_ScreenWidth 
```

## Parameter

nothing; it is a variable, not a call

## Rückgabewert

the width, as a number

## Fehler

the read fails with the reason of the desktop when the screen cannot be measured; the write is refused.

## Beispiel

```ahk
FileAppend A_ScreenWidth, "*"
```
