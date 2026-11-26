# `ComObjFlags`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · COM otomasyonu (ComObject)

## Açıklama

Gets/sets the flags of a COM wrapper.

## Sözdizimi

```ahk
F := ComObjFlags(Obj, Flags, Mask)
```

## Parametreler

- **Obj** — The wrapper.
- **Flags/Mask** — The bits.

## Dönüş değeri

The flags.

## Örnek

```ahk
f := ComObjFlags(o)
```

## İlgili

[`ComObject`](ComObject.md), [`ComObjValue`](ComObjValue.md)
