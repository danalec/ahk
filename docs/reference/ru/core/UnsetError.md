# `UnsetError`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · class · Ядро языка: текст, математика и шаблоны

## Описание

A variable that has not been assigned.

## Синтаксис

```ahk
e := UnsetError(Message, What, Extra)
```

## Параметры

?.

## Возвращаемое значение

An error object (Message/What/Extra/File/Line/Stack).

## Пример

```ahk
throw UnsetError("x")
```

## См. также

[`Error`](Error.md), [`Throw`](Throw.md), [`OnError`](OnError.md)
