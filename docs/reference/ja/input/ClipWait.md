# `ClipWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · ホットキー、ホット文字列、入力

## 説明

Waits for the clipboard to hold data.

## 構文

```ahk
ClipWait Timeout, WaitForAny
```

## パラメータ

0..255.

## 戻り値

Nothing; throws TimeoutError.

## 例

```ahk
ClipWait 2
```

## 関連

[`ClipboardAll`](ClipboardAll.md), [`A_Clipboard`](A_Clipboard.md)
