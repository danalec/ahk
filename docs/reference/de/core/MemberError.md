# `MemberError`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · class · Der Kern der Sprache: Text, Mathematik und Muster

## Beschreibung

A member (property/method) that is not there.

## Syntax

```ahk
e := MemberError(Message, What, Extra)
```

## Parameter

?.

## Rückgabewert

An error object (Message/What/Extra/File/Line/Stack).

## Beispiel

```ahk
throw MemberError("x")
```

## Verwandt

[`Error`](Error.md), [`Throw`](Throw.md), [`OnError`](OnError.md)
