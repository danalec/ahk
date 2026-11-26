# `OnError`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · El analizador y la ejecución del script

## Descripción

Registers the function that handles a failure the script did not catch.

## Sintaxis

```ahk
OnError Function, Add
```

## Parámetros

- **Function** — The callback (Thrown, Mode).
- **Add** — 1/-1.

## Valor de retorno

the integer 1

## Errores

a callback that is not a function object, or a prototype that does not take two parameters, is refused with "Invalid callback function.".

## Ejemplo

```ahk
OnError((err, mode) => 0)
```

## Relacionados

[`OnExit`](OnExit.md), [`Throw`](Throw.md)
