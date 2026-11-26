# `A_ScreenHeight`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · variable · 螢幕擷取與像素函式

## 描述

The height of the screen of the desktop, in pixels.

## 語法

```ahk
A_ScreenHeight 
```

## 參數

nothing; it is a variable, not a call

## 傳回值

the height, as a number

## 錯誤

the read fails with the reason of the desktop when the screen cannot be measured; the write is refused.

## 範例

```ahk
FileAppend A_ScreenHeight, "*"
```
