# `ComObjArray`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · COM 自動化 (ComObject)

## 説明

Creates a COM safe array.

## 構文

```ahk
A := ComObjArray(Type, Counts...)
```

## パラメータ

2..255.

## 戻り値

A COM array.

## 例

```ahk
a := ComObjArray(0xB, 3)
```

## 関連

[`ComObject`](ComObject.md), [`ComValue`](ComValue.md)
