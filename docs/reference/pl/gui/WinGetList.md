# `WinGetList`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · command · Okna GUI i automatyzacja okien

## Opis

The identifiers of every window that matches the title that was given.

## Składnia

```ahk
WinGetList …
```

## Parametry

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## Wartość zwracana

an array of the identifiers, empty when nothing matches

## Błędy

the text of the window and the exclusions are not carried yet; a question the compositor cannot answer fails with its reason.

## Przykład

```ahk
list := WinGetList("Untitled")
```
