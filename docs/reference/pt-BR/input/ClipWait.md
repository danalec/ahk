# `ClipWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Hotkeys, hotstrings e entrada

## Descrição

Waits for the clipboard to hold data.

## Sintaxe

```ahk
ClipWait Timeout, WaitForAny
```

## Parâmetros

0..255.

## Valor de retorno

Nothing; throws TimeoutError.

## Exemplo

```ahk
ClipWait 2
```

## Relacionados

[`ClipboardAll`](ClipboardAll.md), [`A_Clipboard`](A_Clipboard.md)
