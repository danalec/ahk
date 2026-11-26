# `Throw`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*KeySharp* · function · The core of the language: text, math and patterns

## Description

A statement that raises a value. The value is usually an error object, and it may be any value.

## Syntax

```ahk
Throw 
```

## Parameters

Value

## Return value

nothing; the statement leaves the flow of the script

## Errors

none; the value is caught by the catch that names its class.

## Example

```ahk
throw ValueError("boom")
```
