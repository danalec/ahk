# `ComObjValue`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · COM otomasyonu (ComObject)

## Açıklama

Reads the underlying pointer/value of a COM wrapper.

## Sözdizimi

```ahk
P := ComObjValue(Obj)
```

## Parametreler

- **Obj** — The wrapper.

## Dönüş değeri

The pointer or value.

## Örnek

```ahk
p := ComObjValue(o)
```

## İlgili

[`ComObjFlags`](ComObjFlags.md), [`ObjPtr`](ObjPtr.md)
