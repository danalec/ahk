# `ClipboardAll`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · class · 클립보드

## 설명

Reads or writes the whole clipboard (all formats) as binary data.

## 구문

```ahk
Data := ClipboardAll()
```

## 매개변수

?.

## 반환 값

A Buffer of the clipboard's serialized data; assigning it restores the clipboard.

## 설명

On Linux the clipboard carries text; the bytes are the text's bytes.

## 예제

```ahk
saved := ClipboardAll()
```

## 관련

[`OnClipboardChange`](OnClipboardChange.md), [`A_Clipboard`](A_Clipboard.md)
