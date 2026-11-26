# `A_ScreenHeight`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · variable · Screen capture and pixel functions

## Description

The height of the screen of the desktop, in pixels.

## Syntax

```ahk
A_ScreenHeight 
```

## Parameters

nothing; it is a variable, not a call

## Return value

the height, as a number

## Errors

the read fails with the reason of the desktop when the screen cannot be measured; the write is refused.

## Example

```ahk
FileAppend A_ScreenHeight, "*"
```
