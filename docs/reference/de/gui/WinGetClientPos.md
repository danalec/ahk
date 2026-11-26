# `WinGetClientPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · command · GUI-Fenster und Fensterautomatisierung

## Beschreibung

The same place, of the part of the window the program draws in. A Wayland client does not draw its own frame -- the compositor does -- so the geometry the compositor reports is the window the user sees, and it is the answer of both commands.

## Syntax

```ahk
WinGetClientPos …
```

## Parameter

[WindowTitle] [, WinText, &X, &Y, &Width, &Height, ExcludeTitle, ExcludeText]

## Rückgabewert

nothing; the place and the size are written into the variables of the caller

## Fehler

the text of the window and the exclusions are not carried yet; a window that is not there answers 'Target window not found.'

## Beispiel

```ahk
WinGetClientPos "Untitled", , &x, &y, &w, &h
```
