# `DriveGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Makine, süreçler ve ortam

## Açıklama

Reads a property of a drive (List/Capacity/SpaceFree/... ).

## Sözdizimi

```ahk
S := DriveGet(Cmd, Value)
```

## Parametreler

0..255.

## Dönüş değeri

The property.

## Örnek

```ahk
DriveGet("List")
```

## İlgili

[`DriveGetList`](DriveGetList.md), [`DriveGetCapacity`](DriveGetCapacity.md)
