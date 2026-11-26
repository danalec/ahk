# `WinSetTitle`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · GUI windows and window automation

## Description

Renames the window the script made; renaming the window of another program is not something a Wayland client can do.

## Syntax

```ahk
WinSetTitle …
```

## Parameters

NewTitle, WinTitle

## Return value

the empty text

## Errors

"Target window not found." when the script has no window of its own, or when the title does not match the one it has.

## Example

```ahk
WinSetTitle "New Title", "MyWindow"
```
