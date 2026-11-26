# `WinClose`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · GUI pencereleri ve pencere otomasyonu

## Açıklama

Closes a window. It closes the window of the script itself: the listing of the compositor publishes no close request for a window of another program.

## Sözdizimi

```ahk
WinClose WinTitle, WinText, SecondsToWait
```

## Parametreler

- **WinTitle** — The criteria.
- **SecondsToWait** — Then force.

## Dönüş değeri

nothing

## Hatalar

a window that is not there answers 'Target window not found.'; a window of another program cannot be closed this way.

## Örnek

```ahk
WinClose "Untitled"
```

## İlgili

[`WinKill`](WinKill.md), [`WinActivate`](WinActivate.md)
