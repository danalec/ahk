# `ObjGetBase`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · function · Obiekty języka: Array, Map i Func

## Opis

The base of an object: the prototype of its class.

## Składnia

```ahk
ObjGetBase …
```

## Parametry

the object

## Wartość zwracana

the object of the base

## Błędy

a value that is not an object is refused with "Expected an object but got ...".

## Przykład

```ahk
FileAppend ObjGetBase({a: 1}).__Class, "*"
```
