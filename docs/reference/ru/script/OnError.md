# `OnError`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · Разбор и выполнение скрипта

## Описание

Registers the function that handles a failure the script did not catch.

## Синтаксис

```ahk
OnError Function, Add
```

## Параметры

- **Function** — The callback (Thrown, Mode).
- **Add** — 1/-1.

## Возвращаемое значение

the integer 1

## Ошибки

a callback that is not a function object, or a prototype that does not take two parameters, is refused with "Invalid callback function.".

## Пример

```ahk
OnError((err, mode) => 0)
```

## См. также

[`OnExit`](OnExit.md), [`Throw`](Throw.md)
