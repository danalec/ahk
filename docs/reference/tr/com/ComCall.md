# `ComCall`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · function · COM otomasyonu (ComObject)

## Açıklama

Calls a COM method by its vtable index.

## Sözdizimi

```ahk
R := ComCall(Index, Obj, Args..., "ReturnType")
```

## Parametreler

- **Index** — The vtable slot.
- **Obj** — The COM object.
- **Args** — Type/value pairs.

## Dönüş değeri

The return value.

## Notlar

On Linux the D-Bus model has no vtable index; refuses with the reason.

## Örnek

```ahk
ComCall(0, obj, "Ptr")
```

## İlgili

[`ComObject`](ComObject.md), [`ComObjQuery`](ComObjQuery.md)
