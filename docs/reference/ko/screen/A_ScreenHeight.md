# `A_ScreenHeight`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · variable · 화면 캡처와 픽셀 함수

## 설명

The height of the screen of the desktop, in pixels.

## 구문

```ahk
A_ScreenHeight 
```

## 매개변수

nothing; it is a variable, not a call

## 반환 값

the height, as a number

## 오류

the read fails with the reason of the desktop when the screen cannot be measured; the write is refused.

## 예제

```ahk
FileAppend A_ScreenHeight, "*"
```
