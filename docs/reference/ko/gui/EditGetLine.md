# `EditGetLine`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · GUI 창과 창 자동화

## 설명

Reads a line of an edit.

## 구문

```ahk
S := EditGetLine(N, Control, WinTitle)
```

## 매개변수

2..6.

## 반환 값

The line.

## 예제

```ahk
EditGetLine(1, "Edit1")
```

## 관련

[`Gui`](Gui.md), [`ControlGetText`](ControlGetText.md)
