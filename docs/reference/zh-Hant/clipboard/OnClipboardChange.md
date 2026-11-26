# `OnClipboardChange`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · command · 剪貼簿

## 描述

Registers a function to run when the clipboard changes.

## 語法

```ahk
OnClipboardChange Function, Add
```

## 參數

- **Function** — Callback (Type).
- **Add** — 1/-1.

## 傳回值

Nothing.

## 範例

```ahk
OnClipboardChange((t) => ToolTip("clip " t))
```

## 相關

[`ClipboardAll`](ClipboardAll.md), [`A_Clipboard`](A_Clipboard.md)
