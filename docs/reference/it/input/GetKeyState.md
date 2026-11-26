# `GetKeyState`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · function · Tasti rapidi, stringhe di attivazione e input

## Descrizione

Reads what the hotkey watcher saw: whether a key is down. The second parameter carries only the words 'T' and 'P' of v2.

## Sintassi

```ahk
S := GetKeyState(KeyName, Mode)
```

## Parametri

- **KeyName** — The key.
- **Mode** — P physical, T toggle.

## Valore di ritorno

one when the key is down, zero when it is not

## Errori

a key name the tree does not know, and a mode that is neither 'T' nor 'P', are refused as invalid parameters; on a host that is not Linux the watcher cannot answer at all.

## Esempio

```ahk
state := GetKeyState("Shift")
```

## Correlati

[`KeyWait`](KeyWait.md), [`SetCapsLockState`](SetCapsLockState.md)
