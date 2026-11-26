# `UnsetError`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · class · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

A variable that has not been assigned.

## 구문

```ahk
e := UnsetError(Message, What, Extra)
```

## 매개변수

?.

## 반환 값

An error object (Message/What/Extra/File/Line/Stack).

## 예제

```ahk
throw UnsetError("x")
```

## 관련

[`Error`](Error.md), [`Throw`](Throw.md), [`OnError`](OnError.md)
