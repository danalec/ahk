# `OnMessage`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · function · L'analizzatore e l'esecuzione dello script

## Descrizione

Registers a function to handle a window message.

## Sintassi

```ahk
OnMessage MsgNumber, Function, MaxThreads
```

## Parametri

1..3.

## Valore di ritorno

Nothing.

## Esempio

```ahk
OnMessage(0x100, Fn)
```

## Correlati

[`OnExit`](OnExit.md), [`OnError`](OnError.md), [`SendMessage`](SendMessage.md)
