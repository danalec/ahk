# `A_Clipboard`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · variable · Pano

## Açıklama

The text of the clipboard of the desktop. A script may write it, and the concatenation adds to what is there.

## Sözdizimi

```ahk
A_Clipboard 
```

## Parametreler

nothing; it is a variable, not a call

## Dönüş değeri

the text of the clipboard

## Hatalar

the read fails with the reason of the desktop; the write of a number is refused, because the clipboard holds a text.

## Örnek

```ahk
A_Clipboard := "hello"
FileAppend A_Clipboard, "*"
```
