# `MouseGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 熱鍵、熱字串與輸入

## 描述

Reads the place of the pointer this runtime drives, out of the kernel. It is the only pointer on the box whose place can be read at all: the physical mouse is a relative device.

## 語法

```ahk
MouseGetPos OutputX, OutputY, OutputWin, OutputControl, Flag
```

## 參數

- **Output*** — Variables that receive the answers.

## 傳回值

nothing; the place is written into the variables of the caller

## 錯誤

when the kernel will not answer, the desktop answers the place the runtime last put the pointer at, and says so in a note.

## 範例

```ahk
MouseGetPos &x, &y
```

## 相關

[`MouseMove`](MouseMove.md), [`PixelGetColor`](PixelGetColor.md)
