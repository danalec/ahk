# `ExitApp`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · El analizador y la ejecución del script

## Descripción

Ends the script with a code. The code goes into the exit_code of the report.

## Sintaxis

```ahk
ExitApp ExitCode
```

## Parámetros

- **ExitCode** — The code the process exits with (default 0).

## Valor de retorno

the code

## Errores

none.

## Observaciones

OnExit callbacks run before the process ends.

## Ejemplo

```ahk
ExitApp 3
```

## Relacionados

[`Exit`](Exit.md), [`Reload`](Reload.md), [`OnExit`](OnExit.md)
