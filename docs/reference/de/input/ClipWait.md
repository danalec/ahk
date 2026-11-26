# `ClipWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · Hotkeys, Hotstrings und Eingabe

## Beschreibung

Waits for the clipboard to hold data.

## Syntax

```ahk
ClipWait Timeout, WaitForAny
```

## Parameter

0..255.

## Rückgabewert

Nothing; throws TimeoutError.

## Beispiel

```ahk
ClipWait 2
```

## Verwandt

[`ClipboardAll`](ClipboardAll.md), [`A_Clipboard`](A_Clipboard.md)
