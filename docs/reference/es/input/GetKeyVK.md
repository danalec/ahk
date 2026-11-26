# `GetKeyVK`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · command · Hotkeys, hotstrings y entrada

## Descripción

The virtual key code of Windows of the key. It is the number of the key of the keyboard of Windows, and the tree keeps it for the scripts that ask for it.

## Sintaxis

```ahk
GetKeyVK …
```

## Parámetros

KeyName

## Valor de retorno

the virtual key code, a number

## Errores

a name the tree does not know is the empty text, the way v2 answers a name it does not know.

## Ejemplo

```ahk
vk := GetKeyVK("Enter")
```
