# `WinGetCount`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · command · Finestre GUI e automazione delle finestre

## Descrizione

How many windows match the title that was given.

## Sintassi

```ahk
WinGetCount …
```

## Parametri

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## Valore di ritorno

the number of the windows, a number

## Errori

the text of the window and the exclusions are not carried yet; a question the compositor cannot answer fails with its reason.

## Esempio

```ahk
count := WinGetCount("Untitled")
```
