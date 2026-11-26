# `Process`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Makine, süreçler ve ortam

## Açıklama

Operates on a process (v1: Exist/Close/Wait/... ).

## Sözdizimi

```ahk
Process Command, NameOrPID, Value
```

## Parametreler

- **Command** — Exist/Close/Wait/WaitClose/Priority.
- **NameOrPID** — The target.
- **Value** — The parameter.

## Dönüş değeri

The PID or 0.

## Örnek

```ahk
Process "Exist", "notepad.exe"
```

## İlgili

[`ProcessExist`](ProcessExist.md), [`ProcessClose`](ProcessClose.md)
