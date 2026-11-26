# `ClipWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Raccourcis, chaînes de déclenchement et saisie

## Description

Waits for the clipboard to hold data.

## Syntaxe

```ahk
ClipWait Timeout, WaitForAny
```

## Paramètres

0..255.

## Valeur de retour

Nothing; throws TimeoutError.

## Exemple

```ahk
ClipWait 2
```

## Voir aussi

[`ClipboardAll`](ClipboardAll.md), [`A_Clipboard`](A_Clipboard.md)
