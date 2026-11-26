# `ClipWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 熱鍵、熱字串與輸入

## 描述

Waits for the clipboard to hold data.

## 語法

```ahk
ClipWait Timeout, WaitForAny
```

## 參數

0..255.

## 傳回值

Nothing; throws TimeoutError.

## 範例

```ahk
ClipWait 2
```

## 相關

[`ClipboardAll`](ClipboardAll.md), [`A_Clipboard`](A_Clipboard.md)
