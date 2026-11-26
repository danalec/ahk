# `DriveGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · マシン、プロセス、環境

## 説明

Reads a property of a drive (List/Capacity/SpaceFree/... ).

## 構文

```ahk
S := DriveGet(Cmd, Value)
```

## パラメータ

0..255.

## 戻り値

The property.

## 例

```ahk
DriveGet("List")
```

## 関連

[`DriveGetList`](DriveGetList.md), [`DriveGetCapacity`](DriveGetCapacity.md)
