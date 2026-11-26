# `LoadPicture`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · Ekran yakalama ve piksel işlevleri

## Açıklama

Loads a picture file into a handle for Gui/Menu pictures.

## Sözdizimi

```ahk
H := LoadPicture(Filename, Options, ImageType)
```

## Parametreler

- **Filename** — The image.
- **Options** — w/h/icon.
- **ImageType** — Receives the type.

## Dönüş değeri

A handle; 0 if the file is absent (v2.0.31).

## Örnek

```ahk
h := LoadPicture("a.png")
```

## İlgili

[`Gui`](Gui.md), [`ImageSearch`](ImageSearch.md)
