# `LoadPicture`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · 螢幕擷取與像素函式

## 描述

Loads a picture file into a handle for Gui/Menu pictures.

## 語法

```ahk
H := LoadPicture(Filename, Options, ImageType)
```

## 參數

- **Filename** — The image.
- **Options** — w/h/icon.
- **ImageType** — Receives the type.

## 傳回值

A handle; 0 if the file is absent (v2.0.31).

## 範例

```ahk
h := LoadPicture("a.png")
```

## 相關

[`Gui`](Gui.md), [`ImageSearch`](ImageSearch.md)
