# `ImageSearch`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 屏幕捕获与像素函数

## 描述

在屏幕上寻找一张图片。区域只截取一次，然后在其中查找这张图片，带上容差和透明色。

## 语法

```ahk
ImageSearch …
```

## 参数

&X, &Y, X1, Y1, X2, Y2, Picture [, Options]

## 返回值

找到图片时为 1，否则为 0

## 错误

a picture that cannot be loaded is refused with 'Parameter #7 of ImageSearch is invalid.'; on a host without `grim` the search fails with the reason of `grim`.

## 示例

```ahk
ImageSearch &x, &y, 0, 0, 800, 600, "button.png"
```
