# `OnClipboardChange`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · command · Le presse-papiers

## Description

Registers a function to run when the clipboard changes.

## Syntaxe

```ahk
OnClipboardChange Function, Add
```

## Paramètres

- **Function** — Callback (Type).
- **Add** — 1/-1.

## Valeur de retour

Nothing.

## Exemple

```ahk
OnClipboardChange((t) => ToolTip("clip " t))
```

## Voir aussi

[`ClipboardAll`](ClipboardAll.md), [`A_Clipboard`](A_Clipboard.md)
