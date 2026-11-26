# `PixelGetColor`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 屏幕捕获与像素函数

## 描述

读取一个像素的颜色，读的是合成器绘制的画面。后端用 `grim` 截取 1x1 区域，再读出那一个像素。

## 语法

```ahk
C := PixelGetColor(X, Y, Mode)
```

## 参数

- **X/Y** — Screen coords.
- **Mode** — Alt/Slow/RGB.

## 返回值

颜色，格式为 `#rrggbb`

## 错误

a colour written as a number is refused with 'is not a colour: write it as #rrggbb'; on a host without `grim` the read fails with the reason of `grim`.

## 示例

```ahk
colour := PixelGetColor(10, 20)
```

## 相关

[`PixelSearch`](PixelSearch.md), [`ImageSearch`](ImageSearch.md)
