# `ExitApp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Parser i wykonanie skryptu

## Opis

Ends the script with a code. The code goes into the exit_code of the report.

## Składnia

```ahk
ExitApp ExitCode
```

## Parametry

- **ExitCode** — The code the process exits with (default 0).

## Wartość zwracana

the code

## Błędy

none.

## Uwagi

OnExit callbacks run before the process ends.

## Przykład

```ahk
ExitApp 3
```

## Powiązane

[`Exit`](Exit.md), [`Reload`](Reload.md), [`OnExit`](OnExit.md)
