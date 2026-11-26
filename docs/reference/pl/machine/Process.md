# `Process`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Maszyna, procesy i środowisko

## Opis

Operates on a process (v1: Exist/Close/Wait/... ).

## Składnia

```ahk
Process Command, NameOrPID, Value
```

## Parametry

- **Command** — Exist/Close/Wait/WaitClose/Priority.
- **NameOrPID** — The target.
- **Value** — The parameter.

## Wartość zwracana

The PID or 0.

## Przykład

```ahk
Process "Exist", "notepad.exe"
```

## Powiązane

[`ProcessExist`](ProcessExist.md), [`ProcessClose`](ProcessClose.md)
