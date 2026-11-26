# `WinGetCount`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · command · GUI-Fenster und Fensterautomatisierung

## Beschreibung

How many windows match the title that was given.

## Syntax

```ahk
WinGetCount …
```

## Parameter

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## Rückgabewert

the number of the windows, a number

## Fehler

the text of the window and the exclusions are not carried yet; a question the compositor cannot answer fails with its reason.

## Beispiel

```ahk
count := WinGetCount("Untitled")
```
