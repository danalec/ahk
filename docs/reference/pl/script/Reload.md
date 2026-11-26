# `Reload`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Parser i wykonanie skryptu

## Opis

Starts the script again in a process of its own: the old instance ends and the new one carries on with the same file and the same arguments. A script that comes from the standard input has no file, and the v2 does nothing then.

## Składnia

```ahk
Reload
```

## Parametry

none

## Wartość zwracana

the empty text

## Błędy

none; the name takes no parameter, and a script without a file is not refused -- the call simply does nothing.

## Uwagi

The new instance replaces the old; single-instance rules apply.

## Przykład

```ahk
Reload
```

## Powiązane

[`ExitApp`](ExitApp.md), [`#SingleInstance`](#SingleInstance.md)
