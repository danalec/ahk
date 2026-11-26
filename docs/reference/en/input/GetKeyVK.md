# `GetKeyVK`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · command · Hotkeys, hotstrings, and input

## Description

The virtual key code of Windows of the key. It is the number of the key of the keyboard of Windows, and the tree keeps it for the scripts that ask for it.

## Syntax

```ahk
GetKeyVK …
```

## Parameters

KeyName

## Return value

the virtual key code, a number

## Errors

a name the tree does not know is the empty text, the way v2 answers a name it does not know.

## Example

```ahk
vk := GetKeyVK("Enter")
```
