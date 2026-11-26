# `Error`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · class · The core of the language: text, math and patterns

## Description

The base class of the errors of the language.

## Syntax

```ahk
e := Error(Message, What, Extra)
```

## Parameters

?.

## Return value

An error object (Message/What/Extra/File/Line/Stack).

## Example

```ahk
throw Error("x")
```

## Related

[`Error`](Error.md), [`Throw`](Throw.md), [`OnError`](OnError.md)
