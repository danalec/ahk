# `ControlSend`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · GUI 창과 창 자동화

## 설명

Sends keystrokes to a control without activating the window.

## 구문

```ahk
ControlSend Keys, Control, WinTitle
```

## 매개변수

- **Keys** — The keystrokes.
- **Control** — The control (ClassNN).
- **WinTitle** — The window.

## 반환 값

Nothing.

## 예제

```ahk
ControlSend "hello", "Edit1", "ahk_class Notepad"
```

## 관련

[`Send`](Send.md), [`ControlSetText`](ControlSetText.md)
