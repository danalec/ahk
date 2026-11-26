# `WinGetClass`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Finestre GUI e automazione delle finestre

## Descrizione

The class of the window that matches the title that was given.

## Sintassi

```ahk
WinGetClass …
```

## Parametri

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## Valore di ritorno

the class of the window

## Errori

a title that matches no window answers 'Target window not found.'; the text of the window and the exclusions are not carried yet.

## Esempio

```ahk
class := WinGetClass("Untitled")
```
