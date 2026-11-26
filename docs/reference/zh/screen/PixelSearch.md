# `PixelSearch`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 屏幕捕获与像素函数

## 描述

在屏幕上寻找某种颜色的位置。区域只截取一次，然后逐像素查找该颜色。它是不依赖桌面种类的检查手段。

## 语法

```ahk
PixelSearch OutputX, OutputY, X1, Y1, X2, Y2, Color, Variation
```

## 参数

- **Output*** — Receive the found coords.
- **Region** — The rectangle.
- **Color** — 0xRRGGBB.
- **Variation** — 0..255 per channel.

## 返回值

找到颜色时为 1，否则为 0

## 错误

the two places are written into the variables of the caller, and they are made blank when the colour is not there; on a host without `grim` the search fails with the reason of `grim`.

## 示例

```ahk
PixelSearch &x, &y, 0, 0, 100, 100, "#ff0000"
```

## 相关

[`PixelGetColor`](PixelGetColor.md), [`ImageSearch`](ImageSearch.md)
