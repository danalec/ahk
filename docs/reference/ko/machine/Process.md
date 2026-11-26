# `Process`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 머신, 프로세스, 환경

## 설명

Operates on a process (v1: Exist/Close/Wait/... ).

## 구문

```ahk
Process Command, NameOrPID, Value
```

## 매개변수

- **Command** — Exist/Close/Wait/WaitClose/Priority.
- **NameOrPID** — The target.
- **Value** — The parameter.

## 반환 값

The PID or 0.

## 예제

```ahk
Process "Exist", "notepad.exe"
```

## 관련

[`ProcessExist`](ProcessExist.md), [`ProcessClose`](ProcessClose.md)
