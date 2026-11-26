# `Process`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · Die Maschine, die Prozesse und die Umgebung

## Beschreibung

Operates on a process (v1: Exist/Close/Wait/... ).

## Syntax

```ahk
Process Command, NameOrPID, Value
```

## Parameter

- **Command** — Exist/Close/Wait/WaitClose/Priority.
- **NameOrPID** — The target.
- **Value** — The parameter.

## Rückgabewert

The PID or 0.

## Beispiel

```ahk
Process "Exist", "notepad.exe"
```

## Verwandt

[`ProcessExist`](ProcessExist.md), [`ProcessClose`](ProcessClose.md)
