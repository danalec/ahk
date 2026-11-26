# `GetKeySC`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · command · Hotkeys, hotstrings, and input

## Description

The code of the Linux input event of the key. It is the code the box writes to its own device, and not the scan code of the PC keyboard: the two numberings part company on the extended keys.

## Syntax

```ahk
GetKeySC …
```

## Parameters

KeyName

## Return value

the input event code, a number

## Errors

a name the tree does not know is the empty text, the way v2 answers a name it does not know.

## Example

```ahk
sc := GetKeySC("Enter")
```
