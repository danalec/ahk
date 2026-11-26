# `Error`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · class · Dilin çekirdeği: metin, matematik ve kalıplar

## Açıklama

The base class of the errors of the language.

## Sözdizimi

```ahk
e := Error(Message, What, Extra)
```

## Parametreler

?.

## Dönüş değeri

An error object (Message/What/Extra/File/Line/Stack).

## Örnek

```ahk
throw Error("x")
```

## İlgili

[`Error`](Error.md), [`Throw`](Throw.md), [`OnError`](OnError.md)
