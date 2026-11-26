# `OnError`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · function · Parser and script execution

## Description

Registers the function that handles a failure the script did not catch.

## Syntax

```ahk
OnError Function, Add
```

## Parameters

- **Function** — The callback (Thrown, Mode).
- **Add** — 1/-1.

## Return value

the integer 1

## Errors

a callback that is not a function object, or a prototype that does not take two parameters, is refused with "Invalid callback function.".

## Example

```ahk
OnError((err, mode) => 0)
```

## Related

[`OnExit`](OnExit.md), [`Throw`](Throw.md)
