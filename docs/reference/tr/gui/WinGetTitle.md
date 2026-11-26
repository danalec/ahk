# `WinGetTitle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · GUI pencereleri ve pencere otomasyonu

## Açıklama

The title of the active window, or of the window that matches the text that was given.

## Sözdizimi

```ahk
T := WinGetTitle(WinTitle, WinText)
```

## Parametreler

- **WinTitle** — The criteria; omitted = last found.

## Dönüş değeri

the title of the window

## Hatalar

a window that was named and is not there answers 'Target window not found.'; the text of the window and the exclusions are not carried yet.

## Örnek

```ahk
title := WinGetTitle()
```

## İlgili

[`WinGetText`](WinGetText.md), [`WinGetPos`](WinGetPos.md)
