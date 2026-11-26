# `ComObjFlags`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · function · Automatyzacja COM (ComObject)

## Opis

Gets/sets the flags of a COM wrapper.

## Składnia

```ahk
F := ComObjFlags(Obj, Flags, Mask)
```

## Parametry

- **Obj** — The wrapper.
- **Flags/Mask** — The bits.

## Wartość zwracana

The flags.

## Przykład

```ahk
f := ComObjFlags(o)
```

## Powiązane

[`ComObject`](ComObject.md), [`ComObjValue`](ComObjValue.md)
