# `OnMessage`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · El analizador y la ejecución del script

## Descripción

Registers a function to handle a window message.

## Sintaxis

```ahk
OnMessage MsgNumber, Function, MaxThreads
```

## Parámetros

1..3.

## Valor de retorno

Nothing.

## Ejemplo

```ahk
OnMessage(0x100, Fn)
```

## Relacionados

[`OnExit`](OnExit.md), [`OnError`](OnError.md), [`SendMessage`](SendMessage.md)
