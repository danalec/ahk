# `Process`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · マシン、プロセス、環境

## 説明

Operates on a process (v1: Exist/Close/Wait/... ).

## 構文

```ahk
Process Command, NameOrPID, Value
```

## パラメータ

- **Command** — Exist/Close/Wait/WaitClose/Priority.
- **NameOrPID** — The target.
- **Value** — The parameter.

## 戻り値

The PID or 0.

## 例

```ahk
Process "Exist", "notepad.exe"
```

## 関連

[`ProcessExist`](ProcessExist.md), [`ProcessClose`](ProcessClose.md)
