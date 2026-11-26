# `GetKeyVK`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · command · Skróty, ciągi wyzwalające i wprowadzanie

## Opis

The virtual key code of Windows of the key. It is the number of the key of the keyboard of Windows, and the tree keeps it for the scripts that ask for it.

## Składnia

```ahk
GetKeyVK …
```

## Parametry

KeyName

## Wartość zwracana

the virtual key code, a number

## Błędy

a name the tree does not know is the empty text, the way v2 answers a name it does not know.

## Przykład

```ahk
vk := GetKeyVK("Enter")
```
