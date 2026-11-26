# `Floor`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · function · The core of the language: text, math and patterns

## Description

The whole number below a number.

## Syntax

```ahk
N := Floor(Number)
```

## Parameters

- **Number** — The value.

## Return value

the whole number, a float when the number was not whole

## Errors

a value that is not a number is refused.

## Example

```ahk
Floor(1.8)
```

## Related

[`Ceil`](Ceil.md), [`Round`](Round.md)
