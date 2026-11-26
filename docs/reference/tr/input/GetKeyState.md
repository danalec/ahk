# `GetKeyState`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · Kısayollar, tetikleme dizeleri ve giriş

## Açıklama

Reads what the hotkey watcher saw: whether a key is down. The second parameter carries only the words 'T' and 'P' of v2.

## Sözdizimi

```ahk
S := GetKeyState(KeyName, Mode)
```

## Parametreler

- **KeyName** — The key.
- **Mode** — P physical, T toggle.

## Dönüş değeri

one when the key is down, zero when it is not

## Hatalar

a key name the tree does not know, and a mode that is neither 'T' nor 'P', are refused as invalid parameters; on a host that is not Linux the watcher cannot answer at all.

## Örnek

```ahk
state := GetKeyState("Shift")
```

## İlgili

[`KeyWait`](KeyWait.md), [`SetCapsLockState`](SetCapsLockState.md)
