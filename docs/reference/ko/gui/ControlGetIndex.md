# `ControlGetIndex`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · GUI 창과 창 자동화

## 설명

Reads the chosen index of a list.

## 구문

```ahk
N := ControlGetIndex(Control, WinTitle)
```

## 매개변수

1..5.

## 반환 값

The index.

## 예제

```ahk
ControlGetIndex("ListBox1")
```

## 관련

[`ControlSend`](ControlSend.md), [`Gui`](Gui.md)
