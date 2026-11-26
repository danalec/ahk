# `KeyWait`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Hotkeys, hotstrings y entrada

## Descripción

Refused. The runtime does not keep the keys that are down, so it cannot wait for one.

## Sintaxis

```ahk
KeyWait KeyName, Options
```

## Parámetros

- **KeyName** — The key.
- **Options** — D (down), T timeout, L logical.

## Valor de retorno

nothing; the statement fails

## Errores

this runtime does not keep the keys that are down, so it cannot wait for one.

## Ejemplo

```ahk
KeyWait "a"
```

## Relacionados

[`GetKeyState`](GetKeyState.md), [`Hotkey`](Hotkey.md)
