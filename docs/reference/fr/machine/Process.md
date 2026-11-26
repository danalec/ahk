# `Process`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · La machine, les processus et l'environnement

## Description

Operates on a process (v1: Exist/Close/Wait/... ).

## Syntaxe

```ahk
Process Command, NameOrPID, Value
```

## Paramètres

- **Command** — Exist/Close/Wait/WaitClose/Priority.
- **NameOrPID** — The target.
- **Value** — The parameter.

## Valeur de retour

The PID or 0.

## Exemple

```ahk
Process "Exist", "notepad.exe"
```

## Voir aussi

[`ProcessExist`](ProcessExist.md), [`ProcessClose`](ProcessClose.md)
