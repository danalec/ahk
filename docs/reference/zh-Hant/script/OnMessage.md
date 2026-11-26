# `OnMessage`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · 解析器與腳本執行

## 描述

Registers a function to handle a window message.

## 語法

```ahk
OnMessage MsgNumber, Function, MaxThreads
```

## 參數

1..3.

## 傳回值

Nothing.

## 範例

```ahk
OnMessage(0x100, Fn)
```

## 相關

[`OnExit`](OnExit.md), [`OnError`](OnError.md), [`SendMessage`](SendMessage.md)
