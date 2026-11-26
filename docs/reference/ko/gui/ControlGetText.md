# `ControlGetText`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · GUI 창과 창 자동화

## 설명

Reads the text of a control.

## 구문

```ahk
T := ControlGetText(Control, WinTitle)
```

## 매개변수

- **Control** — ClassNN.
- **WinTitle** — The window.

## 반환 값

The text.

## 예제

```ahk
t := ControlGetText("Edit1", "ahk_class Notepad")
```

## 관련

[`ControlSetText`](ControlSetText.md), [`WinGetText`](WinGetText.md)
