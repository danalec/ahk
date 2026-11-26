# `ListViewGetContent`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · command · GUI windows and window automation

## Description

Reads the content of a ListView.

## Syntax

```ahk
S := ListViewGetContent(Options, Control, WinTitle)
```

## Parameters

0..6.

## Return value

The text.

## Example

```ahk
ListViewGetContent("", "SysListView321")
```

## Related

[`Gui`](Gui.md), [`ControlGetText`](ControlGetText.md)
