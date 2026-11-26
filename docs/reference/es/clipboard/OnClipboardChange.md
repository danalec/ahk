# `OnClipboardChange`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · command · El portapapeles

## Descripción

Registers a function to run when the clipboard changes.

## Sintaxis

```ahk
OnClipboardChange Function, Add
```

## Parámetros

- **Function** — Callback (Type).
- **Add** — 1/-1.

## Valor de retorno

Nothing.

## Ejemplo

```ahk
OnClipboardChange((t) => ToolTip("clip " t))
```

## Relacionados

[`ClipboardAll`](ClipboardAll.md), [`A_Clipboard`](A_Clipboard.md)
