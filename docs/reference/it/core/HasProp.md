# `HasProp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · method · `Object` · Il nucleo del linguaggio: testo, matematica e modelli

## Descrizione

Whether an object has a property.

## Sintassi

```ahk
B := HasProp(Obj, Name)
```

## Parametri

- **Obj** — The object.
- **Name** — The property.

## Valore di ritorno

1 when the object has such a property, 0 when it has not

## Errori

nothing; a value that is not an object has the property `__Class` and no other, so the answer for it is one only for that name.

## Esempio

```ahk
o := {a: 1}
FileAppend HasProp(o, "a"), "*"
```

## Correlati

[`HasMethod`](HasMethod.md), [`DefineProp`](DefineProp.md), [`OwnProps`](OwnProps.md)
