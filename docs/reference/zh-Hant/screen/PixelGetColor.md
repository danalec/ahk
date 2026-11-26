# `PixelGetColor`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 螢幕擷取與像素函式

## 描述

The colour of one pixel, read from the frame the compositor draws. The box captures a 1x1 region with `grim` and reads the one pixel.

## 語法

```ahk
C := PixelGetColor(X, Y, Mode)
```

## 參數

- **X/Y** — Screen coords.
- **Mode** — Alt/Slow/RGB.

## 傳回值

the colour, as `#rrggbb`

## 錯誤

a colour written as a number is refused with 'is not a colour: write it as #rrggbb'; on a host without `grim` the read fails with the reason of `grim`.

## 範例

```ahk
colour := PixelGetColor(10, 20)
```

## 相關

[`PixelSearch`](PixelSearch.md), [`ImageSearch`](ImageSearch.md)
