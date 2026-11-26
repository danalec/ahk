# `OnError`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · 解析器與腳本執行

## 描述

Registers the function that handles a failure the script did not catch.

## 語法

```ahk
OnError Function, Add
```

## 參數

- **Function** — The callback (Thrown, Mode).
- **Add** — 1/-1.

## 傳回值

the integer 1

## 錯誤

a callback that is not a function object, or a prototype that does not take two parameters, is refused with "Invalid callback function.".

## 範例

```ahk
OnError((err, mode) => 0)
```

## 相關

[`OnExit`](OnExit.md), [`Throw`](Throw.md)
