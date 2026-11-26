# `WinGetClass`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Okna GUI i automatyzacja okien

## Opis

The class of the window that matches the title that was given.

## Składnia

```ahk
WinGetClass …
```

## Parametry

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## Wartość zwracana

the class of the window

## Błędy

a title that matches no window answers 'Target window not found.'; the text of the window and the exclusions are not carried yet.

## Przykład

```ahk
class := WinGetClass("Untitled")
```
