# `Reload`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · L'analizzatore e l'esecuzione dello script

## Descrizione

Starts the script again in a process of its own: the old instance ends and the new one carries on with the same file and the same arguments. A script that comes from the standard input has no file, and the v2 does nothing then.

## Sintassi

```ahk
Reload
```

## Parametri

none

## Valore di ritorno

the empty text

## Errori

none; the name takes no parameter, and a script without a file is not refused -- the call simply does nothing.

## Osservazioni

The new instance replaces the old; single-instance rules apply.

## Esempio

```ahk
Reload
```

## Correlati

[`ExitApp`](ExitApp.md), [`#SingleInstance`](#SingleInstance.md)
