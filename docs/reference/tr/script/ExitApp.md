# `ExitApp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Ayrıştırıcı ve betiğin çalıştırılması

## Açıklama

Ends the script with a code. The code goes into the exit_code of the report.

## Sözdizimi

```ahk
ExitApp ExitCode
```

## Parametreler

- **ExitCode** — The code the process exits with (default 0).

## Dönüş değeri

the code

## Hatalar

none.

## Notlar

OnExit callbacks run before the process ends.

## Örnek

```ahk
ExitApp 3
```

## İlgili

[`Exit`](Exit.md), [`Reload`](Reload.md), [`OnExit`](OnExit.md)
