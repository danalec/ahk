# `WinGetTitle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Finestre GUI e automazione delle finestre

## Descrizione

The title of the active window, or of the window that matches the text that was given.

## Sintassi

```ahk
T := WinGetTitle(WinTitle, WinText)
```

## Parametri

- **WinTitle** — The criteria; omitted = last found.

## Valore di ritorno

the title of the window

## Errori

a window that was named and is not there answers 'Target window not found.'; the text of the window and the exclusions are not carried yet.

## Esempio

```ahk
title := WinGetTitle()
```

## Correlati

[`WinGetText`](WinGetText.md), [`WinGetPos`](WinGetPos.md)
