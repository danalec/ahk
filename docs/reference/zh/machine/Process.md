# `Process`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 机器、进程与环境

## 描述

Operates on a process (v1: Exist/Close/Wait/... ).

## 语法

```ahk
Process Command, NameOrPID, Value
```

## 参数

- **Command** — Exist/Close/Wait/WaitClose/Priority.
- **NameOrPID** — The target.
- **Value** — The parameter.

## 返回值

The PID or 0.

## 示例

```ahk
Process "Exist", "notepad.exe"
```

## 相关

[`ProcessExist`](ProcessExist.md), [`ProcessClose`](ProcessClose.md)
