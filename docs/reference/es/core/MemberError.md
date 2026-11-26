# `MemberError`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · class · El núcleo del lenguaje: texto, matemáticas y patrones

## Descripción

A member (property/method) that is not there.

## Sintaxis

```ahk
e := MemberError(Message, What, Extra)
```

## Parámetros

?.

## Valor de retorno

An error object (Message/What/Extra/File/Line/Stack).

## Ejemplo

```ahk
throw MemberError("x")
```

## Relacionados

[`Error`](Error.md), [`Throw`](Throw.md), [`OnError`](OnError.md)
