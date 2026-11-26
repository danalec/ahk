# `DriveGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 機器、行程與環境

## 描述

Reads a property of a drive (List/Capacity/SpaceFree/... ).

## 語法

```ahk
S := DriveGet(Cmd, Value)
```

## 參數

0..255.

## 傳回值

The property.

## 範例

```ahk
DriveGet("List")
```

## 相關

[`DriveGetList`](DriveGetList.md), [`DriveGetCapacity`](DriveGetCapacity.md)
