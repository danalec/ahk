# `ClipWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Skróty, ciągi wyzwalające i wprowadzanie

## Opis

Waits for the clipboard to hold data.

## Składnia

```ahk
ClipWait Timeout, WaitForAny
```

## Parametry

0..255.

## Wartość zwracana

Nothing; throws TimeoutError.

## Przykład

```ahk
ClipWait 2
```

## Powiązane

[`ClipboardAll`](ClipboardAll.md), [`A_Clipboard`](A_Clipboard.md)
