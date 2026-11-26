# `A_ScreenWidth`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · variable · Przechwytywanie ekranu i funkcje pikseli

## Opis

The width of the screen of the desktop, in pixels.

## Składnia

```ahk
A_ScreenWidth 
```

## Parametry

nothing; it is a variable, not a call

## Wartość zwracana

the width, as a number

## Błędy

the read fails with the reason of the desktop when the screen cannot be measured; the write is refused.

## Przykład

```ahk
FileAppend A_ScreenWidth, "*"
```
