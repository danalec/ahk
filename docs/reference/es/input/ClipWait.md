# `ClipWait`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Hotkeys, hotstrings y entrada

## Descripción

Waits for the clipboard to hold data.

## Sintaxis

```ahk
ClipWait Timeout, WaitForAny
```

## Parámetros

0..255.

## Valor de retorno

Nothing; throws TimeoutError.

## Ejemplo

```ahk
ClipWait 2
```

## Relacionados

[`ClipboardAll`](ClipboardAll.md), [`A_Clipboard`](A_Clipboard.md)
