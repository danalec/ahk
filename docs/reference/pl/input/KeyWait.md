# `KeyWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Skróty, ciągi wyzwalające i wprowadzanie

## Opis

Refused. The runtime does not keep the keys that are down, so it cannot wait for one.

## Składnia

```ahk
KeyWait KeyName, Options
```

## Parametry

- **KeyName** — The key.
- **Options** — D (down), T timeout, L logical.

## Wartość zwracana

nothing; the statement fails

## Błędy

this runtime does not keep the keys that are down, so it cannot wait for one.

## Przykład

```ahk
KeyWait "a"
```

## Powiązane

[`GetKeyState`](GetKeyState.md), [`Hotkey`](Hotkey.md)
