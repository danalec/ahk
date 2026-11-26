# `MouseClick`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 熱鍵、熱字串與輸入

## 描述

Clicks with the pointer of the backend. The button is the first parameter, the way the function of v2 takes it, and a button that is not there is the left one.

## 語法

```ahk
MouseClick …
```

## 參數

Button, X, Y [Count] [Options]

## 傳回值

nothing

## 錯誤

the wheel, the fourth and the fifth buttons are refused as not carried yet.

## 範例

```ahk
MouseClick "Right", 100, 200
```
