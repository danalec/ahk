# `ComObjActive`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · COM otomasyonu (ComObject)

## Açıklama

Gets a running COM object by CLSID (the ROT).

## Sözdizimi

```ahk
Obj := ComObjActive(CLSID)
```

## Parametreler

- **CLSID** — The class id.

## Dönüş değeri

The running object.

## Örnek

```ahk
o := ComObjActive("{...}")
```

## İlgili

[`ComObjGet`](ComObjGet.md), [`ComObject`](ComObject.md)
