# `WinGetTransColor`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · GUI 창과 창 자동화

## 설명

The colour a window lets through: the colour key of a layered window, the one that is not drawn. The colour is written as "0xRRGGBB", in the order of the bits of the red, the green and the blue. A window that is not layered answers the empty text.

## 구문

```ahk
WinGetTransColor …
```

## 매개변수

WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## 반환 값

the colour as "0xRRGGBB", or the empty text when the window is not layered

## 오류

a window that is not there refuses with 'Target window not found.', and the title that was asked about is in the `Extra`; the text of the window and the two exclusions are not carried yet.

## 예제

```ahk
FileAppend WinGetTransColor("ZZZ_NO_WINDOW_ZZZ"), "*"
```
