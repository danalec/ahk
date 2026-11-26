# `HasMethod`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · method · `Object` · The core of the language: text, math and patterns

## Description

Whether an object has a method.

## Syntax

```ahk
HasMethod 
```

## Parameters

the object, the name of the method, and the number of parameters it would take

## Return value

1 when the object has such a method, 0 when it has not

## Errors

nothing; a value that is not an object answers zero, and the question with no name asks whether the value is a class.

## Example

```ahk
o := {a: 1}
FileAppend HasMethod(o, "a"), "*"
```
