# `MouseGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · ホットキー、ホット文字列、入力

## 説明

Reads the place of the pointer this runtime drives, out of the kernel. It is the only pointer on the box whose place can be read at all: the physical mouse is a relative device.

## 構文

```ahk
MouseGetPos OutputX, OutputY, OutputWin, OutputControl, Flag
```

## パラメータ

- **Output*** — Variables that receive the answers.

## 戻り値

nothing; the place is written into the variables of the caller

## エラー

when the kernel will not answer, the desktop answers the place the runtime last put the pointer at, and says so in a note.

## 例

```ahk
MouseGetPos &x, &y
```

## 関連

[`MouseMove`](MouseMove.md), [`PixelGetColor`](PixelGetColor.md)
