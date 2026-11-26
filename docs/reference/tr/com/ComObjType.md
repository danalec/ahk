# `ComObjType`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · COM otomasyonu (ComObject)

## Açıklama

Reads the type/name of a COM object.

## Sözdizimi

```ahk
S := ComObjType(Obj, What)
```

## Parametreler

- **Obj** — The object.
- **What** — Name/Class/IID/...

## Dönüş değeri

The requested string.

## Örnek

```ahk
n := ComObjType(o, "Name")
```

## İlgili

[`ComObject`](ComObject.md), [`ComObjValue`](ComObjValue.md)
