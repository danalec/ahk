# `WinGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · GUI-Fenster und Fensterautomatisierung

## Beschreibung

Where the window is and how big it is, as the compositor reports it.

## Syntax

```ahk
WinGetPos X, Y, W, H, WinTitle
```

## Parameter

- **Output*** — Variables.
- **WinTitle** — The criteria.

## Rückgabewert

nothing; the place and the size are written into the variables of the caller

## Fehler

the text of the window and the exclusions are not carried yet; a window that is not there answers 'Target window not found.'

## Beispiel

```ahk
WinGetPos "Untitled", , &x, &y, &w, &h
```

## Verwandt

[`WinMove`](WinMove.md), [`WinGetTitle`](WinGetTitle.md)
