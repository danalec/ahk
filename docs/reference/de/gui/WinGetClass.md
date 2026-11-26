# `WinGetClass`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · GUI-Fenster und Fensterautomatisierung

## Beschreibung

The class of the window that matches the title that was given.

## Syntax

```ahk
WinGetClass …
```

## Parameter

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## Rückgabewert

the class of the window

## Fehler

a title that matches no window answers 'Target window not found.'; the text of the window and the exclusions are not carried yet.

## Beispiel

```ahk
class := WinGetClass("Untitled")
```
