# `WinMinimize`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · GUI-Fenster und Fensterautomatisierung

## Beschreibung

Minimizes the window the script made, and only that one: a window of another program cannot be minimized from a Wayland client.

## Syntax

```ahk
WinMinimize …
```

## Parameter

WinTitle

## Rückgabewert

the empty text

## Fehler

"Target window not found." when the script has no window of its own, or when the title does not match the one it has; the reason of the desktop when the helper refuses the state.

## Beispiel

```ahk
WinMinimize "MyWindow"
```
