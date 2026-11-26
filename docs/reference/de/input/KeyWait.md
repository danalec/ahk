# `KeyWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · Hotkeys, Hotstrings und Eingabe

## Beschreibung

Refused. The runtime does not keep the keys that are down, so it cannot wait for one.

## Syntax

```ahk
KeyWait KeyName, Options
```

## Parameter

- **KeyName** — The key.
- **Options** — D (down), T timeout, L logical.

## Rückgabewert

nothing; the statement fails

## Fehler

this runtime does not keep the keys that are down, so it cannot wait for one.

## Beispiel

```ahk
KeyWait "a"
```

## Verwandt

[`GetKeyState`](GetKeyState.md), [`Hotkey`](Hotkey.md)
