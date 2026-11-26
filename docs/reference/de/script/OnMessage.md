# `OnMessage`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · function · Der Parser und die Ausführung des Skripts

## Beschreibung

Registers a function to handle a window message.

## Syntax

```ahk
OnMessage MsgNumber, Function, MaxThreads
```

## Parameter

1..3.

## Rückgabewert

Nothing.

## Beispiel

```ahk
OnMessage(0x100, Fn)
```

## Verwandt

[`OnExit`](OnExit.md), [`OnError`](OnError.md), [`SendMessage`](SendMessage.md)
