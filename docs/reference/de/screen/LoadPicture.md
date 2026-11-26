# `LoadPicture`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · function · Bildschirmaufnahme und Pixel-Funktionen

## Beschreibung

Loads a picture file into a handle for Gui/Menu pictures.

## Syntax

```ahk
H := LoadPicture(Filename, Options, ImageType)
```

## Parameter

- **Filename** — The image.
- **Options** — w/h/icon.
- **ImageType** — Receives the type.

## Rückgabewert

A handle; 0 if the file is absent (v2.0.31).

## Beispiel

```ahk
h := LoadPicture("a.png")
```

## Verwandt

[`Gui`](Gui.md), [`ImageSearch`](ImageSearch.md)
