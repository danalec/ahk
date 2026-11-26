# `LoadPicture`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · Captura de pantalla y funciones de píxel

## Descripción

Loads a picture file into a handle for Gui/Menu pictures.

## Sintaxis

```ahk
H := LoadPicture(Filename, Options, ImageType)
```

## Parámetros

- **Filename** — The image.
- **Options** — w/h/icon.
- **ImageType** — Receives the type.

## Valor de retorno

A handle; 0 if the file is absent (v2.0.31).

## Ejemplo

```ahk
h := LoadPicture("a.png")
```

## Relacionados

[`Gui`](Gui.md), [`ImageSearch`](ImageSearch.md)
