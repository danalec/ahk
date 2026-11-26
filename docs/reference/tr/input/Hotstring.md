# `Hotstring`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · Kısayollar, tetikleme dizeleri ve giriş

## Açıklama

Registers a text that fires a function when it is typed. The text of v2 is written between two of the same character, and the character is not part of it.

## Sözdizimi

```ahk
Hotstring …
```

## Parametreler

Text, Action [, Options]

## Dönüş değeri

nothing

## Hatalar

a text that is not a string is refused as an invalid parameter; the watcher reads the physical keyboard, and the device of the box is skipped so that a hotstring does not fire on what the script types.

## Örnek

```ahk
Hotstring "::btw::", ShowTheDefinition
```
