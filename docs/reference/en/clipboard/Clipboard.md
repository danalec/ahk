# `Clipboard`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · variable · The clipboard

## Description

The variable of the clipboard; the v2 spells it A_Clipboard and the v1 spells it Clipboard, and both are the same text. It is read with wl-paste on a Wayland session and with xclip or xsel on an X11 one, and written with wl-copy or xclip.

## Syntax

```ahk
Clipboard 
```

## Parameters

--

## Return value

the text on the clipboard

## Errors

an empty clipboard is a value and not a failure; an X11 session with neither xclip nor xsel answers "the clipboard of an X11 session cannot be read: neither xclip nor xsel is installed on this machine"; a write calls the function registered for OnClipboardChange with 1, because the change the runtime can see is its own.

## Example

```ahk
text := A_Clipboard
```
