# `LastError`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · variable · 네이티브 함수 호출 (DllCall)

## 설명

The last error code of the machine (the Win32 GetLastError).

## 구문

```ahk
N := LastError
```

## 매개변수

?.

## 반환 값

The code.

## 예제

```ahk
e := LastError
```

## 관련

[`DllCall`](DllCall.md), [`OSError`](OSError.md)
