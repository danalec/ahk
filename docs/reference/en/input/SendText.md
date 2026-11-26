# `SendText`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · command · Hotkeys, hotstrings, and input

## Description

Types the text as it stands: the braces of v2 are not read here, and the text goes to the keyboard one character at a time.

## Syntax

```ahk
SendText …
```

## Parameters

Text

## Return value

nothing

## Errors

the keyboard of the backend is a Linux uinput device, so on a host that is not Linux the text is not typed.

## Example

```ahk
SendText "a literal {braced} text"
```
