# `OnMessage`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · function · Parser i wykonanie skryptu

## Opis

Registers a function to handle a window message.

## Składnia

```ahk
OnMessage MsgNumber, Function, MaxThreads
```

## Parametry

1..3.

## Wartość zwracana

Nothing.

## Przykład

```ahk
OnMessage(0x100, Fn)
```

## Powiązane

[`OnExit`](OnExit.md), [`OnError`](OnError.md), [`SendMessage`](SendMessage.md)
