# `A_DD`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · variable · File and directory operations

## Description

The day of the month of the local time, in two digits.

## Syntax

```ahk
A_DD 
```

## Parameters

nothing; it is a variable, not a call

## Return value

the day, as a text of two digits

## Errors

the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

## Example

```ahk
FileAppend A_DD, "*"
```
