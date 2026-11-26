# `A_KeyDelay`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · variable · Hotkeys, hotstrings, and input

## Description

The delay between the keystrokes of a `Send`, in milliseconds. It is a setting of the run, and it is ten by default.

## Syntax

```ahk
A_KeyDelay 
```

## Parameters

nothing; it is a variable, not a call

## Return value

the delay, as a number

## Errors

the write takes the number of the value, and a text that holds no number is zero.

## Example

```ahk
FileAppend A_KeyDelay, "*"
```
