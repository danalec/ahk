# `Buffer`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · class · 네이티브 함수 호출 (DllCall)

## 설명

Allocates a block of bytes.

## 구문

```ahk
Buf := Buffer(Size, Fill)
```

## 매개변수

- **Size** — Bytes.
- **Fill** — Initial byte value.

## 반환 값

A Buffer with Ptr/Size/__Item.

## 예제

```ahk
b := Buffer(16, 0)
```

## 관련

[`NumGet`](NumGet.md), [`NumPut`](NumPut.md), [`StrGet`](StrGet.md)
