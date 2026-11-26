# `Integer`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · class · The core of the language: text, math and patterns

## Description

Converts a value to an integer.

## Syntax

```ahk
N := Integer(Value)
```

## Parameters

- **Value** — A number or numeric text.

## Return value

The integer; throws TypeError if not convertible.

## Example

```ahk
Integer("42")  ; 42
```

## Related

[`Float`](Float.md), [`String`](String.md), [`Number`](Number.md)
