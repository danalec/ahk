# `WinGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Finestre GUI e automazione delle finestre

## Descrizione

Where the window is and how big it is, as the compositor reports it.

## Sintassi

```ahk
WinGetPos X, Y, W, H, WinTitle
```

## Parametri

- **Output*** — Variables.
- **WinTitle** — The criteria.

## Valore di ritorno

nothing; the place and the size are written into the variables of the caller

## Errori

the text of the window and the exclusions are not carried yet; a window that is not there answers 'Target window not found.'

## Esempio

```ahk
WinGetPos "Untitled", , &x, &y, &w, &h
```

## Correlati

[`WinMove`](WinMove.md), [`WinGetTitle`](WinGetTitle.md)
