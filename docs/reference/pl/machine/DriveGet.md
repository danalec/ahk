# `DriveGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Maszyna, procesy i środowisko

## Opis

Reads a property of a drive (List/Capacity/SpaceFree/... ).

## Składnia

```ahk
S := DriveGet(Cmd, Value)
```

## Parametry

0..255.

## Wartość zwracana

The property.

## Przykład

```ahk
DriveGet("List")
```

## Powiązane

[`DriveGetList`](DriveGetList.md), [`DriveGetCapacity`](DriveGetCapacity.md)
