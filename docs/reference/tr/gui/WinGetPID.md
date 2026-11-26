# `WinGetPID`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · command · GUI pencereleri ve pencere otomasyonu

## Açıklama

Answers the process of a window, through the accessibility bus of the session: the registry of the bus names the applications, a window is a child of one, and the bus answers its PID.

## Sözdizimi

```ahk
WinGetPID …
```

## Parametreler

WinTitle

## Dönüş değeri

the pid of the process, as an integer

## Hatalar

"Target window not found." when no window has that title; an empty title is refused with "the window is found by its title, and an empty title is not carried yet."; the text of the window and the exclusions are refused with "the text of the window and the exclusions are not carried yet."

## Örnek

```ahk
pid := WinGetPID("Calculator")
```
