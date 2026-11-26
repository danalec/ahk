# `OnClipboardChange`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · command · クリップボード

## 説明

Registers a function to run when the clipboard changes.

## 構文

```ahk
OnClipboardChange Function, Add
```

## パラメータ

- **Function** — Callback (Type).
- **Add** — 1/-1.

## 戻り値

Nothing.

## 例

```ahk
OnClipboardChange((t) => ToolTip("clip " t))
```

## 関連

[`ClipboardAll`](ClipboardAll.md), [`A_Clipboard`](A_Clipboard.md)
