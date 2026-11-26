# `OnMessage`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · 파서와 스크립트 실행

## 설명

Registers a function to handle a window message.

## 구문

```ahk
OnMessage MsgNumber, Function, MaxThreads
```

## 매개변수

1..3.

## 반환 값

Nothing.

## 예제

```ahk
OnMessage(0x100, Fn)
```

## 관련

[`OnExit`](OnExit.md), [`OnError`](OnError.md), [`SendMessage`](SendMessage.md)
