# `OnClipboardChange`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · 剪贴板

## 描述

Registers a function to run when the clipboard changes.

## 语法

```ahk
OnClipboardChange Function, Add
```

## 参数

- **Function** — Callback (Type).
- **Add** — 1/-1.

## 返回值

Nothing.

## 示例

```ahk
OnClipboardChange((t) => ToolTip("clip " t))
```

## 相关

[`ClipboardAll`](ClipboardAll.md), [`A_Clipboard`](A_Clipboard.md)
