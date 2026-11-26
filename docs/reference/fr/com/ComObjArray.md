# `ComObjArray`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · L'automatisation COM (ComObject)

## Description

Creates a COM safe array.

## Syntaxe

```ahk
A := ComObjArray(Type, Counts...)
```

## Paramètres

2..255.

## Valeur de retour

A COM array.

## Exemple

```ahk
a := ComObjArray(0xB, 3)
```

## Voir aussi

[`ComObject`](ComObject.md), [`ComValue`](ComValue.md)
