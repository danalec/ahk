# `Error`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · class · Rdzeń języka: tekst, matematyka i wzorce

## Opis

The base class of the errors of the language.

## Składnia

```ahk
e := Error(Message, What, Extra)
```

## Parametry

?.

## Wartość zwracana

An error object (Message/What/Extra/File/Line/Stack).

## Przykład

```ahk
throw Error("x")
```

## Powiązane

[`Error`](Error.md), [`Throw`](Throw.md), [`OnError`](OnError.md)
