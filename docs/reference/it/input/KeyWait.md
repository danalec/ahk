# `KeyWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Tasti rapidi, stringhe di attivazione e input

## Descrizione

Refused. The runtime does not keep the keys that are down, so it cannot wait for one.

## Sintassi

```ahk
KeyWait KeyName, Options
```

## Parametri

- **KeyName** — The key.
- **Options** — D (down), T timeout, L logical.

## Valore di ritorno

nothing; the statement fails

## Errori

this runtime does not keep the keys that are down, so it cannot wait for one.

## Esempio

```ahk
KeyWait "a"
```

## Correlati

[`GetKeyState`](GetKeyState.md), [`Hotkey`](Hotkey.md)
