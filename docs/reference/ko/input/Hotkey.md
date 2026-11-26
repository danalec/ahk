# `Hotkey`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 핫키, 핫스트링, 입력

## 설명

Registers a hotkey: the key it watches for and the function it runs. The runtime owns the registry and the machine owns the watcher: the script says which function a hotkey runs, and the machine says which key was pressed.

## 구문

```ahk
Hotkey KeyName, Action, Options
```

## 매개변수

- **KeyName** — e.g. ^!a, ~LButton.
- **Action** — A function/label, or On/Off/Toggle/AltTab etc.
- **Options** — On/Off/UseErrorLevel/MaxThreadsPerHotkey.

## 반환 값

nothing

## 오류

a hotkey without an action is 'Nonexistent hotkey.'; the watcher reads the physical keyboard of the machine, so on a host that is not Linux the registration fails with 'the hotkeys of this box watch a Linux device; this is not Linux'.

## 설명

On Linux the keyboard is watched (/dev/input or X11); on Windows RegisterHotKey or the hook.

## 예제

```ahk
Hotkey "^s", SaveTheFile
```

## 관련

[`Hotstring`](Hotstring.md), [`#HotkeyInterval`](#HotkeyInterval.md), [`KeyWait`](KeyWait.md)
