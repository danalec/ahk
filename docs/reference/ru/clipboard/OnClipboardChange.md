# `OnClipboardChange`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · command · Буфер обмена

## Описание

Registers a function to run when the clipboard changes.

## Синтаксис

```ahk
OnClipboardChange Function, Add
```

## Параметры

- **Function** — Callback (Type).
- **Add** — 1/-1.

## Возвращаемое значение

Nothing.

## Пример

```ahk
OnClipboardChange((t) => ToolTip("clip " t))
```

## См. также

[`ClipboardAll`](ClipboardAll.md), [`A_Clipboard`](A_Clipboard.md)
