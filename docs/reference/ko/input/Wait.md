# `Wait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · method · `InputHook` · 핫키, 핫스트링, 입력

## 설명

Waits for the input to end.

## 구문

```ahk
S := InputHook.Wait(MaxSeconds)
```

## 매개변수

?.

## 반환 값

The EndReason.

## 예제

```ahk
ih.Wait()
```

## 관련

[`InputHook`](InputHook.md), [`Start`](Start.md), [`Stop`](Stop.md)
