# `HasBase`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · method · `Object` · The core of the language: text, math and patterns

## Description

Whether the base of an object is another object.

## Syntax

```ahk
HasBase 
```

## Parameters

the object, and the base

## Return value

1 when the base is that one, 0 when it is not

## Errors

a base that is not an object is refused before the question of the object is asked.

## Example

```ahk
FileAppend HasBase({a: 1}, {}), "*"
```
