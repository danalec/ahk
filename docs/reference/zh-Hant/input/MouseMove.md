# `MouseMove`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 熱鍵、熱字串與輸入

## 描述

Moves the pointer of the backend to a place on the screen. The box moves the pointer to the place, and not from where it is.

## 語法

```ahk
MouseMove X, Y, Speed
```

## 參數

- **X/Y** — Screen coords (or relative with R).
- **Speed** — 0..100.

## 傳回值

nothing

## 錯誤

the relative place of the move and the speed of the move are refused as not carried yet.

## 範例

```ahk
MouseMove 400, 300
```

## 相關

[`Click`](Click.md), [`MouseGetPos`](MouseGetPos.md)
