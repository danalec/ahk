# `ExitApp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · L'analizzatore e l'esecuzione dello script

## Descrizione

Ends the script with a code. The code goes into the exit_code of the report.

## Sintassi

```ahk
ExitApp ExitCode
```

## Parametri

- **ExitCode** — The code the process exits with (default 0).

## Valore di ritorno

the code

## Errori

none.

## Osservazioni

OnExit callbacks run before the process ends.

## Esempio

```ahk
ExitApp 3
```

## Correlati

[`Exit`](Exit.md), [`Reload`](Reload.md), [`OnExit`](OnExit.md)
