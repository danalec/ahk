# `ClipWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 핫키, 핫스트링, 입력

## 설명

Waits for the clipboard to hold data.

## 구문

```ahk
ClipWait Timeout, WaitForAny
```

## 매개변수

0..255.

## 반환 값

Nothing; throws TimeoutError.

## 예제

```ahk
ClipWait 2
```

## 관련

[`ClipboardAll`](ClipboardAll.md), [`A_Clipboard`](A_Clipboard.md)
