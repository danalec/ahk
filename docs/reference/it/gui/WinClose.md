# `WinClose`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Finestre GUI e automazione delle finestre

## Descrizione

Closes a window. It closes the window of the script itself: the listing of the compositor publishes no close request for a window of another program.

## Sintassi

```ahk
WinClose WinTitle, WinText, SecondsToWait
```

## Parametri

- **WinTitle** — The criteria.
- **SecondsToWait** — Then force.

## Valore di ritorno

nothing

## Errori

a window that is not there answers 'Target window not found.'; a window of another program cannot be closed this way.

## Esempio

```ahk
WinClose "Untitled"
```

## Correlati

[`WinKill`](WinKill.md), [`WinActivate`](WinActivate.md)
