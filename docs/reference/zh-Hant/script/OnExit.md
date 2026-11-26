# `OnExit`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · command · 解析器與腳本執行

## 描述

Registers the function that runs when the script ends, whatever ended it: the last statement, a code the script asked to stop with, or a failure that stopped it.

## 語法

```ahk
OnExit Function, Add
```

## 參數

- **Function** — The callback (ExitReason, ExitCode).
- **Add** — 1 to add, -1 to remove.

## 傳回值

the integer 1

## 錯誤

a callback that is not a function object, or a prototype that does not take two parameters, is refused with "Invalid callback function.".

## 範例

```ahk
OnExit((reason, code) => 0)
```

## 相關

[`ExitApp`](ExitApp.md), [`OnError`](OnError.md)
