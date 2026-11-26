# `LoopRegKey`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · variable · 레지스트리 API

## 설명

The name of the current subkey in a Loop Reg.

## 구문

```ahk
S := LoopRegKey
```

## 매개변수

?.

## 반환 값

The subkey name.

## 예제

```ahk
Loop Reg, "HKCU\Software"
    s := LoopRegKey
```

## 관련

[`LoopRegName`](LoopRegName.md), [`LoopRegType`](LoopRegType.md)
