# `ImageSearch`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 화면 캡처와 픽셀 함수

## 설명

Looks for a picture on the screen. The region is captured once and the picture is looked for in it, with the variation and the transparent colour.

## 구문

```ahk
ImageSearch …
```

## 매개변수

&X, &Y, X1, Y1, X2, Y2, Picture [, Options]

## 반환 값

one when the picture is there, zero when it is not

## 오류

a picture that cannot be loaded is refused with 'Parameter #7 of ImageSearch is invalid.'; on a host without `grim` the search fails with the reason of `grim`.

## 예제

```ahk
ImageSearch &x, &y, 0, 0, 800, 600, "button.png"
```
