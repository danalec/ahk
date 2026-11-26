# `KeyWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Kısayollar, tetikleme dizeleri ve giriş

## Açıklama

Refused. The runtime does not keep the keys that are down, so it cannot wait for one.

## Sözdizimi

```ahk
KeyWait KeyName, Options
```

## Parametreler

- **KeyName** — The key.
- **Options** — D (down), T timeout, L logical.

## Dönüş değeri

nothing; the statement fails

## Hatalar

this runtime does not keep the keys that are down, so it cannot wait for one.

## Örnek

```ahk
KeyWait "a"
```

## İlgili

[`GetKeyState`](GetKeyState.md), [`Hotkey`](Hotkey.md)
