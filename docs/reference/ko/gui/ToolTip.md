# `ToolTip`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · GUI 창과 창 자동화

## 설명

Shows a small window of text at a place of the screen. Called with no text it takes the window away.

## 구문

```ahk
ToolTip Text, X, Y, WhichToolTip
```

## 매개변수

- **Text** — The text; omitted hides.
- **X/Y** — Position.
- **WhichToolTip** — 1..20.

## 반환 값

the number that identifies the window, or the empty text when the window is taken away

## 오류

the desktop reports the failure of the call.

## 예제

```ahk
ToolTip "hello", 10, 10
```

## 관련

[`TrayTip`](TrayTip.md), [`MsgBox`](MsgBox.md)
