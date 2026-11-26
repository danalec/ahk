# `UnsetItemError`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · class · Il nucleo del linguaggio: testo, matematica e modelli

## Descrizione

An item of a collection that is unset.

## Sintassi

```ahk
e := UnsetItemError(Message, What, Extra)
```

## Parametri

?.

## Valore di ritorno

An error object (Message/What/Extra/File/Line/Stack).

## Esempio

```ahk
throw UnsetItemError("x")
```

## Correlati

[`Error`](Error.md), [`Throw`](Throw.md), [`OnError`](OnError.md)
