# `HasProp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · method · `Object` · Rdzeń języka: tekst, matematyka i wzorce

## Opis

Whether an object has a property.

## Składnia

```ahk
B := HasProp(Obj, Name)
```

## Parametry

- **Obj** — The object.
- **Name** — The property.

## Wartość zwracana

1 when the object has such a property, 0 when it has not

## Błędy

nothing; a value that is not an object has the property `__Class` and no other, so the answer for it is one only for that name.

## Przykład

```ahk
o := {a: 1}
FileAppend HasProp(o, "a"), "*"
```

## Powiązane

[`HasMethod`](HasMethod.md), [`DefineProp`](DefineProp.md), [`OwnProps`](OwnProps.md)
