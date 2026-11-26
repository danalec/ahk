# `OnMessage`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · O analisador e a execução do script

## Descrição

Registers a function to handle a window message.

## Sintaxe

```ahk
OnMessage MsgNumber, Function, MaxThreads
```

## Parâmetros

1..3.

## Valor de retorno

Nothing.

## Exemplo

```ahk
OnMessage(0x100, Fn)
```

## Relacionados

[`OnExit`](OnExit.md), [`OnError`](OnError.md), [`SendMessage`](SendMessage.md)
