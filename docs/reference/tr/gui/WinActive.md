# `WinActive`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · GUI pencereleri ve pencere otomasyonu

## Açıklama

The window that has the focus. The box is a Wayland session, and the compositor is the one that knows the windows and which of them is in front.

## Sözdizimi

```ahk
WinActive …
```

## Parametreler

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## Dönüş değeri

the identifier of the active window, zero when there is none

## Hatalar

the text of the window and the exclusions are not carried yet; a compositor that does not publish the focus answers with the reason, and not with a guess.

## Örnek

```ahk
id := WinActive()
```
