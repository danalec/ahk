# `ComObjArray`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · COM otomasyonu (ComObject)

## Açıklama

Creates a COM safe array.

## Sözdizimi

```ahk
A := ComObjArray(Type, Counts...)
```

## Parametreler

2..255.

## Dönüş değeri

A COM array.

## Örnek

```ahk
a := ComObjArray(0xB, 3)
```

## İlgili

[`ComObject`](ComObject.md), [`ComValue`](ComValue.md)
