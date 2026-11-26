# `LoadPicture`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 屏幕捕获与像素函数

## 描述

Loads a picture file into a handle for Gui/Menu pictures.

## 语法

```ahk
H := LoadPicture(Filename, Options, ImageType)
```

## 参数

- **Filename** — The image.
- **Options** — w/h/icon.
- **ImageType** — Receives the type.

## 返回值

A handle; 0 if the file is absent (v2.0.31).

## 示例

```ahk
h := LoadPicture("a.png")
```

## 相关

[`Gui`](Gui.md), [`ImageSearch`](ImageSearch.md)
