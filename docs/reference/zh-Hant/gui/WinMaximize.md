# `WinMaximize`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · GUI 視窗與視窗自動化

## 描述

Maximizes the window the script made, and only that one; the helper of the box is the one that knows how to do it.

## 語法

```ahk
WinMaximize …
```

## 參數

WinTitle

## 傳回值

the empty text

## 錯誤

"Target window not found." when the script has no window of its own, or when the title does not match the one it has.

## 範例

```ahk
WinMaximize "MyWindow"
```
