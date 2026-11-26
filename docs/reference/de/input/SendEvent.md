# `SendEvent`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · command · Hotkeys, Hotstrings und Eingabe

## Beschreibung

Sends the keys the way the box sends them. The event method of v2 is the method of Windows, and the tree has one keyboard, so the keys go through it the way `Send` sends them.

## Syntax

```ahk
SendEvent …
```

## Parameter

Keys

## Rückgabewert

nothing

## Fehler

the keyboard of the backend is a Linux uinput device, and it fails on a host that is not Linux.

## Beispiel

```ahk
SendEvent "^a"
```
