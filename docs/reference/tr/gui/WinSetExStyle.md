# `WinSetExStyle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · command · GUI pencereleri ve pencere otomasyonu

## Açıklama

Writes the extended style of a window. The style is absolute, the way the style of a window is: the text may carry a `+`, a `-` or a `^` before the number, and the runtime computes the new extended style out of the one that is there.

## Sözdizimi

```ahk
WinSetExStyle …
```

## Parametreler

Style, WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## Dönüş değeri

the empty text

## Hatalar

an empty text refuses with 'Parameter #1 of WinSetExStyle is invalid.', and 'empty string' is in the `Extra`; a window that is not there refuses with 'Target window not found.'.

## Örnek

```ahk
WinSetExStyle("+0x8", "ZZZ_NO_WINDOW_ZZZ")
```
