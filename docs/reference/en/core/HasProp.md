# `HasProp`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · method · `Object` · The core of the language: text, math and patterns

## Description

Whether an object has a property.

## Syntax

```ahk
B := HasProp(Obj, Name)
```

## Parameters

- **Obj** — The object.
- **Name** — The property.

## Return value

1 when the object has such a property, 0 when it has not

## Errors

nothing; a value that is not an object has the property `__Class` and no other, so the answer for it is one only for that name.

## Example

```ahk
o := {a: 1}
FileAppend HasProp(o, "a"), "*"
```

## Related

[`HasMethod`](HasMethod.md), [`DefineProp`](DefineProp.md), [`OwnProps`](OwnProps.md)
