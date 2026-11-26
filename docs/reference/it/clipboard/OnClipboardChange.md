# `OnClipboardChange`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · command · Gli appunti

## Descrizione

Registers a function to run when the clipboard changes.

## Sintassi

```ahk
OnClipboardChange Function, Add
```

## Parametri

- **Function** — Callback (Type).
- **Add** — 1/-1.

## Valore di ritorno

Nothing.

## Esempio

```ahk
OnClipboardChange((t) => ToolTip("clip " t))
```

## Correlati

[`ClipboardAll`](ClipboardAll.md), [`A_Clipboard`](A_Clipboard.md)
