# `GetKeyState`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · 핫키, 핫스트링, 입력

## 설명

Reads what the hotkey watcher saw: whether a key is down. The second parameter carries only the words 'T' and 'P' of v2.

## 구문

```ahk
S := GetKeyState(KeyName, Mode)
```

## 매개변수

- **KeyName** — The key.
- **Mode** — P physical, T toggle.

## 반환 값

one when the key is down, zero when it is not

## 오류

a key name the tree does not know, and a mode that is neither 'T' nor 'P', are refused as invalid parameters; on a host that is not Linux the watcher cannot answer at all.

## 예제

```ahk
state := GetKeyState("Shift")
```

## 관련

[`KeyWait`](KeyWait.md), [`SetCapsLockState`](SetCapsLockState.md)
