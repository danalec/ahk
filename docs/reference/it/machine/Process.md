# `Process`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · La macchina, i processi e l'ambiente

## Descrizione

Operates on a process (v1: Exist/Close/Wait/... ).

## Sintassi

```ahk
Process Command, NameOrPID, Value
```

## Parametri

- **Command** — Exist/Close/Wait/WaitClose/Priority.
- **NameOrPID** — The target.
- **Value** — The parameter.

## Valore di ritorno

The PID or 0.

## Esempio

```ahk
Process "Exist", "notepad.exe"
```

## Correlati

[`ProcessExist`](ProcessExist.md), [`ProcessClose`](ProcessClose.md)
