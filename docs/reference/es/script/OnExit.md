# `OnExit`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · command · El analizador y la ejecución del script

## Descripción

Registers the function that runs when the script ends, whatever ended it: the last statement, a code the script asked to stop with, or a failure that stopped it.

## Sintaxis

```ahk
OnExit Function, Add
```

## Parámetros

- **Function** — The callback (ExitReason, ExitCode).
- **Add** — 1 to add, -1 to remove.

## Valor de retorno

the integer 1

## Errores

a callback that is not a function object, or a prototype that does not take two parameters, is refused with "Invalid callback function.".

## Ejemplo

```ahk
OnExit((reason, code) => 0)
```

## Relacionados

[`ExitApp`](ExitApp.md), [`OnError`](OnError.md)
