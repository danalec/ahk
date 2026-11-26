# `OnExit`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · command · Ayrıştırıcı ve betiğin çalıştırılması

## Açıklama

Registers the function that runs when the script ends, whatever ended it: the last statement, a code the script asked to stop with, or a failure that stopped it.

## Sözdizimi

```ahk
OnExit Function, Add
```

## Parametreler

- **Function** — The callback (ExitReason, ExitCode).
- **Add** — 1 to add, -1 to remove.

## Dönüş değeri

the integer 1

## Hatalar

a callback that is not a function object, or a prototype that does not take two parameters, is refused with "Invalid callback function.".

## Örnek

```ahk
OnExit((reason, code) => 0)
```

## İlgili

[`ExitApp`](ExitApp.md), [`OnError`](OnError.md)
