# `ImageSearch`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 螢幕擷取與像素函式

## 描述

Looks for a picture on the screen. The region is captured once and the picture is looked for in it, with the variation and the transparent colour.

## 語法

```ahk
ImageSearch …
```

## 參數

&X, &Y, X1, Y1, X2, Y2, Picture [, Options]

## 傳回值

one when the picture is there, zero when it is not

## 錯誤

a picture that cannot be loaded is refused with 'Parameter #7 of ImageSearch is invalid.'; on a host without `grim` the search fails with the reason of `grim`.

## 範例

```ahk
ImageSearch &x, &y, 0, 0, 800, 600, "button.png"
```
