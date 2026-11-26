# `GetKeyVK`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · command · Kısayollar, tetikleme dizeleri ve giriş

## Açıklama

The virtual key code of Windows of the key. It is the number of the key of the keyboard of Windows, and the tree keeps it for the scripts that ask for it.

## Sözdizimi

```ahk
GetKeyVK …
```

## Parametreler

KeyName

## Dönüş değeri

the virtual key code, a number

## Hatalar

a name the tree does not know is the empty text, the way v2 answers a name it does not know.

## Örnek

```ahk
vk := GetKeyVK("Enter")
```
