# `PixelGetColor`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 화면 캡처와 픽셀 함수

## 설명

The colour of one pixel, read from the frame the compositor draws. The box captures a 1x1 region with `grim` and reads the one pixel.

## 구문

```ahk
C := PixelGetColor(X, Y, Mode)
```

## 매개변수

- **X/Y** — Screen coords.
- **Mode** — Alt/Slow/RGB.

## 반환 값

the colour, as `#rrggbb`

## 오류

a colour written as a number is refused with 'is not a colour: write it as #rrggbb'; on a host without `grim` the read fails with the reason of `grim`.

## 예제

```ahk
colour := PixelGetColor(10, 20)
```

## 관련

[`PixelSearch`](PixelSearch.md), [`ImageSearch`](ImageSearch.md)
