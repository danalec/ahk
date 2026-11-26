# `WinMaximize`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · GUI pencereleri ve pencere otomasyonu

## Açıklama

Maximizes the window the script made, and only that one; the helper of the box is the one that knows how to do it.

## Sözdizimi

```ahk
WinMaximize …
```

## Parametreler

WinTitle

## Dönüş değeri

the empty text

## Hatalar

"Target window not found." when the script has no window of its own, or when the title does not match the one it has.

## Örnek

```ahk
WinMaximize "MyWindow"
```
