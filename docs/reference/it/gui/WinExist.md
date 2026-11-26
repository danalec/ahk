# `WinExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · function · Finestre GUI e automazione delle finestre

## Descrizione

Whether a window with this title exists. The windows are the ones of the compositor, and a number names one of them.

## Sintassi

```ahk
HWND := WinExist(WinTitle, WinText, ExcludeTitle, ExcludeText)
```

## Parametri

- **WinTitle** — The criteria.

## Valore di ritorno

the identifier of the window that is there, zero when there is none

## Errori

a window that is not there is an answer and not a failure; a question the compositor cannot answer fails with its reason.

## Osservazioni

Sets the last-found window for the Win* family.

## Esempio

```ahk
if WinExist("Untitled")
```

## Correlati

[`WinActive`](WinActive.md), [`WinActivate`](WinActivate.md)
