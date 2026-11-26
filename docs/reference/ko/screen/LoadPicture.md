# `LoadPicture`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · 화면 캡처와 픽셀 함수

## 설명

Loads a picture file into a handle for Gui/Menu pictures.

## 구문

```ahk
H := LoadPicture(Filename, Options, ImageType)
```

## 매개변수

- **Filename** — The image.
- **Options** — w/h/icon.
- **ImageType** — Receives the type.

## 반환 값

A handle; 0 if the file is absent (v2.0.31).

## 예제

```ahk
h := LoadPicture("a.png")
```

## 관련

[`Gui`](Gui.md), [`ImageSearch`](ImageSearch.md)
