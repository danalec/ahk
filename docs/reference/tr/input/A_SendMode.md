# `A_SendMode`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · variable · Kısayollar, tetikleme dizeleri ve giriş

## Açıklama

The method that a `Send` uses: `Input`, `Event`, `Play` or `InputThenPlay`.

## Sözdizimi

```ahk
A_SendMode 
```

## Parametreler

nothing; it is a variable, not a call

## Dönüş değeri

the name of the mode, as a text

## Hatalar

the write takes the name of one of the four modes, and any other name is refused with "Invalid value.".

## Örnek

```ahk
FileAppend A_SendMode, "*"
```
