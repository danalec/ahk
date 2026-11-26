# `WinClose`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Okna GUI i automatyzacja okien

## Opis

Closes a window. It closes the window of the script itself: the listing of the compositor publishes no close request for a window of another program.

## Składnia

```ahk
WinClose WinTitle, WinText, SecondsToWait
```

## Parametry

- **WinTitle** — The criteria.
- **SecondsToWait** — Then force.

## Wartość zwracana

nothing

## Błędy

a window that is not there answers 'Target window not found.'; a window of another program cannot be closed this way.

## Przykład

```ahk
WinClose "Untitled"
```

## Powiązane

[`WinKill`](WinKill.md), [`WinActivate`](WinActivate.md)
