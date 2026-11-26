# `OnExit`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · command · L'analizzatore e l'esecuzione dello script

## Descrizione

Registers the function that runs when the script ends, whatever ended it: the last statement, a code the script asked to stop with, or a failure that stopped it.

## Sintassi

```ahk
OnExit Function, Add
```

## Parametri

- **Function** — The callback (ExitReason, ExitCode).
- **Add** — 1 to add, -1 to remove.

## Valore di ritorno

the integer 1

## Errori

a callback that is not a function object, or a prototype that does not take two parameters, is refused with "Invalid callback function.".

## Esempio

```ahk
OnExit((reason, code) => 0)
```

## Correlati

[`ExitApp`](ExitApp.md), [`OnError`](OnError.md)
