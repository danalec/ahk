# `A_SendMode`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · variable · Hotkeys, hotstrings, and input

## Description

The method that a `Send` uses: `Input`, `Event`, `Play` or `InputThenPlay`.

## Syntax

```ahk
A_SendMode 
```

## Parameters

nothing; it is a variable, not a call

## Return value

the name of the mode, as a text

## Errors

the write takes the name of one of the four modes, and any other name is refused with "Invalid value.".

## Example

```ahk
FileAppend A_SendMode, "*"
```
