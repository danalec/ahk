# `Process`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 機器、行程與環境

## 描述

Operates on a process (v1: Exist/Close/Wait/... ).

## 語法

```ahk
Process Command, NameOrPID, Value
```

## 參數

- **Command** — Exist/Close/Wait/WaitClose/Priority.
- **NameOrPID** — The target.
- **Value** — The parameter.

## 傳回值

The PID or 0.

## 範例

```ahk
Process "Exist", "notepad.exe"
```

## 相關

[`ProcessExist`](ProcessExist.md), [`ProcessClose`](ProcessClose.md)
