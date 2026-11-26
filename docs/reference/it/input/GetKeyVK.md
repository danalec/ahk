# `GetKeyVK`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · command · Tasti rapidi, stringhe di attivazione e input

## Descrizione

The virtual key code of Windows of the key. It is the number of the key of the keyboard of Windows, and the tree keeps it for the scripts that ask for it.

## Sintassi

```ahk
GetKeyVK …
```

## Parametri

KeyName

## Valore di ritorno

the virtual key code, a number

## Errori

a name the tree does not know is the empty text, the way v2 answers a name it does not know.

## Esempio

```ahk
vk := GetKeyVK("Enter")
```
