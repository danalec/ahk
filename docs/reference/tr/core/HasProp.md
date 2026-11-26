# `HasProp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · method · `Object` · Dilin çekirdeği: metin, matematik ve kalıplar

## Açıklama

Whether an object has a property.

## Sözdizimi

```ahk
B := HasProp(Obj, Name)
```

## Parametreler

- **Obj** — The object.
- **Name** — The property.

## Dönüş değeri

1 when the object has such a property, 0 when it has not

## Hatalar

nothing; a value that is not an object has the property `__Class` and no other, so the answer for it is one only for that name.

## Örnek

```ahk
o := {a: 1}
FileAppend HasProp(o, "a"), "*"
```

## İlgili

[`HasMethod`](HasMethod.md), [`DefineProp`](DefineProp.md), [`OwnProps`](OwnProps.md)
