# `OnMessage`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · Ayrıştırıcı ve betiğin çalıştırılması

## Açıklama

Registers a function to handle a window message.

## Sözdizimi

```ahk
OnMessage MsgNumber, Function, MaxThreads
```

## Parametreler

1..3.

## Dönüş değeri

Nothing.

## Örnek

```ahk
OnMessage(0x100, Fn)
```

## İlgili

[`OnExit`](OnExit.md), [`OnError`](OnError.md), [`SendMessage`](SendMessage.md)
