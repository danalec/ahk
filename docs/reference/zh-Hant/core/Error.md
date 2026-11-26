# `Error`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · class · 語言核心：文字、數學與模式

## 描述

The base class of the errors of the language.

## 語法

```ahk
e := Error(Message, What, Extra)
```

## 參數

?.

## 傳回值

An error object (Message/What/Extra/File/Line/Stack).

## 範例

```ahk
throw Error("x")
```

## 相關

[`Error`](Error.md), [`Throw`](Throw.md), [`OnError`](OnError.md)
