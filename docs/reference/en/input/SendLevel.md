# `SendLevel`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · Hotkeys, hotstrings, and input

## Description

Carries the level of the events the script sends, and answers the level that was there before. The level tells the hotkeys which of the artificial events they are to ignore, and nothing reads it yet.

## Syntax

```ahk
SendLevel …
```

## Parameters

Level

## Return value

the level that was there before, a number

## Errors

a level outside 0 to 100 is refused as an invalid parameter.

## Example

```ahk
SendLevel 1
```
