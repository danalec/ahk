# `KeyWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Горячие клавиши, горячие строки и ввод

## Описание

Refused. The runtime does not keep the keys that are down, so it cannot wait for one.

## Синтаксис

```ahk
KeyWait KeyName, Options
```

## Параметры

- **KeyName** — The key.
- **Options** — D (down), T timeout, L logical.

## Возвращаемое значение

nothing; the statement fails

## Ошибки

this runtime does not keep the keys that are down, so it cannot wait for one.

## Пример

```ahk
KeyWait "a"
```

## См. также

[`GetKeyState`](GetKeyState.md), [`Hotkey`](Hotkey.md)
