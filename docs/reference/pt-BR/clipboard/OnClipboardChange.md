# `OnClipboardChange`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · A área de transferência

## Descrição

Registers a function to run when the clipboard changes.

## Sintaxe

```ahk
OnClipboardChange Function, Add
```

## Parâmetros

- **Function** — Callback (Type).
- **Add** — 1/-1.

## Valor de retorno

Nothing.

## Exemplo

```ahk
OnClipboardChange((t) => ToolTip("clip " t))
```

## Relacionados

[`ClipboardAll`](ClipboardAll.md), [`A_Clipboard`](A_Clipboard.md)
