# `GetKeyName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · command · Kısayollar, tetikleme dizeleri ve giriş

## Açıklama

The name of the key, the way v2 gives it. The names are the ones of the key list of v2: `Esc`, `Enter`, `CtrlRight`, `Numpad0`, and the `vkXX` and `scXXX` spellings.

## Sözdizimi

```ahk
GetKeyName …
```

## Parametreler

KeyName

## Dönüş değeri

the name of the key

## Hatalar

a name the tree does not know is the empty text, the way v2 answers a name it does not know.

## Örnek

```ahk
name := GetKeyName("Enter")
```
