# `WinGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · GUI pencereleri ve pencere otomasyonu

## Açıklama

Where the window is and how big it is, as the compositor reports it.

## Sözdizimi

```ahk
WinGetPos X, Y, W, H, WinTitle
```

## Parametreler

- **Output*** — Variables.
- **WinTitle** — The criteria.

## Dönüş değeri

nothing; the place and the size are written into the variables of the caller

## Hatalar

the text of the window and the exclusions are not carried yet; a window that is not there answers 'Target window not found.'

## Örnek

```ahk
WinGetPos "Untitled", , &x, &y, &w, &h
```

## İlgili

[`WinMove`](WinMove.md), [`WinGetTitle`](WinGetTitle.md)
