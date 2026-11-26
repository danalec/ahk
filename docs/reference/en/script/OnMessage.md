# `OnMessage`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · function · Parser and script execution

## Description

Registers a function to handle a window message.

## Syntax

```ahk
OnMessage MsgNumber, Function, MaxThreads
```

## Parameters

1..3.

## Return value

Nothing.

## Example

```ahk
OnMessage(0x100, Fn)
```

## Related

[`OnExit`](OnExit.md), [`OnError`](OnError.md), [`SendMessage`](SendMessage.md)
