# `Object`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · Dilin nesneleri: Array, Map ve Func

## Açıklama

The root of the chain of the classes: every object of the runtime is one. `Object()` makes an object with nothing in it, and the literal `{a: 1}` makes one with a property.

## Sözdizimi

```ahk
Obj := Object(Props...)
```

## Parametreler

- **Props** — Alternating names and values.

## Dönüş değeri

the object of the class `Object`

## Hatalar

nothing; a member that is not there is refused with the type of the value and the name of the member.

## Notlar

Base is Object; OwnProps/HasProp/DefineProp/DeleteProp; __Get/__Set/__Call meta-functions.

## Örnek

```ahk
o := Object()
FileAppend o.__Class, "*"
```

## İlgili

[`Map`](Map.md), [`Array`](Array.md), [`Gui`](Gui.md)
