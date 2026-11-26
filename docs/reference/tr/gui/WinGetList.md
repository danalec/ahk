# `WinGetList`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · command · GUI pencereleri ve pencere otomasyonu

## Açıklama

The identifiers of every window that matches the title that was given.

## Sözdizimi

```ahk
WinGetList …
```

## Parametreler

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## Dönüş değeri

an array of the identifiers, empty when nothing matches

## Hatalar

the text of the window and the exclusions are not carried yet; a question the compositor cannot answer fails with its reason.

## Örnek

```ahk
list := WinGetList("Untitled")
```
