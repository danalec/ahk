# `Ptr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · method · `Buffer` · 네이티브 함수 호출 (DllCall)

## 설명

The pointer type marker for DllCall/NumGet (the machine's pointer width).

## 구문

```ahk
Ptr
```

## 매개변수

?.

## 반환 값

Used as a type name; 8 bytes on 64-bit, 4 on 32-bit.

## 예제

```ahk
NumGet(buf, 0, "Ptr")
```

## 관련

[`PtrSize`](PtrSize.md), [`Buffer`](Buffer.md), [`DllCall`](DllCall.md)
