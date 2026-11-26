# `A_Clipboard`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · variable · The clipboard

## Description

The text of the clipboard of the desktop. A script may write it, and the concatenation adds to what is there.

## Syntax

```ahk
A_Clipboard 
```

## Parameters

nothing; it is a variable, not a call

## Return value

the text of the clipboard

## Errors

the read fails with the reason of the desktop; the write of a number is refused, because the clipboard holds a text.

## Example

```ahk
A_Clipboard := "hello"
FileAppend A_Clipboard, "*"
```
