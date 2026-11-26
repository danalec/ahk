# `WinHide`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · GUI windows and window automation

## Description

Takes the window of the script off the screen.

## Syntax

```ahk
WinHide …
```

## Parameters

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## Return value

nothing

## Errors

a window that is not there answers 'Target window not found.'; a window of another program cannot be hidden this way.

## Example

```ahk
WinHide "Untitled"
```
