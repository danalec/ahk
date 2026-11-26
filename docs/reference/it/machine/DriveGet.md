# `DriveGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · La macchina, i processi e l'ambiente

## Descrizione

Reads a property of a drive (List/Capacity/SpaceFree/... ).

## Sintassi

```ahk
S := DriveGet(Cmd, Value)
```

## Parametri

0..255.

## Valore di ritorno

The property.

## Esempio

```ahk
DriveGet("List")
```

## Correlati

[`DriveGetList`](DriveGetList.md), [`DriveGetCapacity`](DriveGetCapacity.md)
