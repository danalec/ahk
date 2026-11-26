# `KeyWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 핫키, 핫스트링, 입력

## 설명

Refused. The runtime does not keep the keys that are down, so it cannot wait for one.

## 구문

```ahk
KeyWait KeyName, Options
```

## 매개변수

- **KeyName** — The key.
- **Options** — D (down), T timeout, L logical.

## 반환 값

nothing; the statement fails

## 오류

this runtime does not keep the keys that are down, so it cannot wait for one.

## 예제

```ahk
KeyWait "a"
```

## 관련

[`GetKeyState`](GetKeyState.md), [`Hotkey`](Hotkey.md)
