# `WinMaximize`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · GUI-Fenster und Fensterautomatisierung

## Beschreibung

Maximizes the window the script made, and only that one; the helper of the box is the one that knows how to do it.

## Syntax

```ahk
WinMaximize …
```

## Parameter

WinTitle

## Rückgabewert

the empty text

## Fehler

"Target window not found." when the script has no window of its own, or when the title does not match the one it has.

## Beispiel

```ahk
WinMaximize "MyWindow"
```
