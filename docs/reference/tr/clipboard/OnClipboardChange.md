# `OnClipboardChange`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · command · Pano

## Açıklama

Registers a function to run when the clipboard changes.

## Sözdizimi

```ahk
OnClipboardChange Function, Add
```

## Parametreler

- **Function** — Callback (Type).
- **Add** — 1/-1.

## Dönüş değeri

Nothing.

## Örnek

```ahk
OnClipboardChange((t) => ToolTip("clip " t))
```

## İlgili

[`ClipboardAll`](ClipboardAll.md), [`A_Clipboard`](A_Clipboard.md)
