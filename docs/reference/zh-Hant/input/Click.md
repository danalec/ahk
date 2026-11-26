# `Click`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 熱鍵、熱字串與輸入

## 描述

Clicks with the pointer of the backend, which is an absolute uinput device: it moves to the place, settles, presses and releases.

## 語法

```ahk
Click Options
```

## 參數

- **Options** — X Y, button, count, Down/Up, Rel.

## 傳回值

nothing

## 錯誤

the wheel, the fourth and the fifth buttons, the pressing of a button and the place relative to the pointer are refused as not carried yet; on a host without `grim` the click fails with the reason of the screen, because the pointer needs the size of the screen.

## 備註

On Linux the absolute uinput pointer / XTEST.

## 範例

```ahk
Click 100, 200
```

## 相關

[`MouseMove`](MouseMove.md), [`MouseClick`](MouseClick.md), [`Send`](Send.md)
