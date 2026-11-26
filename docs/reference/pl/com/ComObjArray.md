# `ComObjArray`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · function · Automatyzacja COM (ComObject)

## Opis

Creates a COM safe array.

## Składnia

```ahk
A := ComObjArray(Type, Counts...)
```

## Parametry

2..255.

## Wartość zwracana

A COM array.

## Przykład

```ahk
a := ComObjArray(0xB, 3)
```

## Powiązane

[`ComObject`](ComObject.md), [`ComValue`](ComValue.md)
