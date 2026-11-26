# `LoopRegTimeModified`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · variable · 레지스트리 API

## 설명

The last-write time of the current registry key.

## 구문

```ahk
S := LoopRegTimeModified
```

## 매개변수

?.

## 반환 값

A YYYYMMDDHH24MISS timestamp.

## 예제

```ahk
t := LoopRegTimeModified
```

## 관련

[`LoopRegKey`](LoopRegKey.md), [`FormatTime`](FormatTime.md)
