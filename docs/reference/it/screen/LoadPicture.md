# `LoadPicture`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · function · Cattura dello schermo e funzioni sui pixel

## Descrizione

Loads a picture file into a handle for Gui/Menu pictures.

## Sintassi

```ahk
H := LoadPicture(Filename, Options, ImageType)
```

## Parametri

- **Filename** — The image.
- **Options** — w/h/icon.
- **ImageType** — Receives the type.

## Valore di ritorno

A handle; 0 if the file is absent (v2.0.31).

## Esempio

```ahk
h := LoadPicture("a.png")
```

## Correlati

[`Gui`](Gui.md), [`ImageSearch`](ImageSearch.md)
