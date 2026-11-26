# `OnClipboardChange`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · 클립보드

## 설명

Registers a function to run when the clipboard changes.

## 구문

```ahk
OnClipboardChange Function, Add
```

## 매개변수

- **Function** — Callback (Type).
- **Add** — 1/-1.

## 반환 값

Nothing.

## 예제

```ahk
OnClipboardChange((t) => ToolTip("clip " t))
```

## 관련

[`ClipboardAll`](ClipboardAll.md), [`A_Clipboard`](A_Clipboard.md)
