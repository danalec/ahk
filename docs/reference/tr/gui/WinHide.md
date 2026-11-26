# `WinHide`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · GUI pencereleri ve pencere otomasyonu

## Açıklama

Takes the window of the script off the screen.

## Sözdizimi

```ahk
WinHide …
```

## Parametreler

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## Dönüş değeri

nothing

## Hatalar

a window that is not there answers 'Target window not found.'; a window of another program cannot be hidden this way.

## Örnek

```ahk
WinHide "Untitled"
```
