# `KeyWait`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · Hotkeys, hotstrings, and input

## Description

Refused. The runtime does not keep the keys that are down, so it cannot wait for one.

## Syntax

```ahk
KeyWait KeyName, Options
```

## Parameters

- **KeyName** — The key.
- **Options** — D (down), T timeout, L logical.

## Return value

nothing; the statement fails

## Errors

this runtime does not keep the keys that are down, so it cannot wait for one.

## Example

```ahk
KeyWait "a"
```

## Related

[`GetKeyState`](GetKeyState.md), [`Hotkey`](Hotkey.md)
