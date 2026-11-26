# `MouseGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 핫키, 핫스트링, 입력

## 설명

Reads the place of the pointer this runtime drives, out of the kernel. It is the only pointer on the box whose place can be read at all: the physical mouse is a relative device.

## 구문

```ahk
MouseGetPos OutputX, OutputY, OutputWin, OutputControl, Flag
```

## 매개변수

- **Output*** — Variables that receive the answers.

## 반환 값

nothing; the place is written into the variables of the caller

## 오류

when the kernel will not answer, the desktop answers the place the runtime last put the pointer at, and says so in a note.

## 예제

```ahk
MouseGetPos &x, &y
```

## 관련

[`MouseMove`](MouseMove.md), [`PixelGetColor`](PixelGetColor.md)
