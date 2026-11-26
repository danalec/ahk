# `OnExit`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · command · Der Parser und die Ausführung des Skripts

## Beschreibung

Registers the function that runs when the script ends, whatever ended it: the last statement, a code the script asked to stop with, or a failure that stopped it.

## Syntax

```ahk
OnExit Function, Add
```

## Parameter

- **Function** — The callback (ExitReason, ExitCode).
- **Add** — 1 to add, -1 to remove.

## Rückgabewert

the integer 1

## Fehler

a callback that is not a function object, or a prototype that does not take two parameters, is refused with "Invalid callback function.".

## Beispiel

```ahk
OnExit((reason, code) => 0)
```

## Verwandt

[`ExitApp`](ExitApp.md), [`OnError`](OnError.md)
