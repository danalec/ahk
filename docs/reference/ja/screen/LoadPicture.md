# `LoadPicture`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · 画面キャプチャとピクセル関数

## 説明

Loads a picture file into a handle for Gui/Menu pictures.

## 構文

```ahk
H := LoadPicture(Filename, Options, ImageType)
```

## パラメータ

- **Filename** — The image.
- **Options** — w/h/icon.
- **ImageType** — Receives the type.

## 戻り値

A handle; 0 if the file is absent (v2.0.31).

## 例

```ahk
h := LoadPicture("a.png")
```

## 関連

[`Gui`](Gui.md), [`ImageSearch`](ImageSearch.md)
