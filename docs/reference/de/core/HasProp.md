# `HasProp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · method · `Object` · Der Kern der Sprache: Text, Mathematik und Muster

## Beschreibung

Whether an object has a property.

## Syntax

```ahk
B := HasProp(Obj, Name)
```

## Parameter

- **Obj** — The object.
- **Name** — The property.

## Rückgabewert

1 when the object has such a property, 0 when it has not

## Fehler

nothing; a value that is not an object has the property `__Class` and no other, so the answer for it is one only for that name.

## Beispiel

```ahk
o := {a: 1}
FileAppend HasProp(o, "a"), "*"
```

## Verwandt

[`HasMethod`](HasMethod.md), [`DefineProp`](DefineProp.md), [`OwnProps`](OwnProps.md)
