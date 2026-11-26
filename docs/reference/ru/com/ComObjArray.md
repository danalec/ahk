# `ComObjArray`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · COM-автоматизация (ComObject)

## Описание

Creates a COM safe array.

## Синтаксис

```ahk
A := ComObjArray(Type, Counts...)
```

## Параметры

2..255.

## Возвращаемое значение

A COM array.

## Пример

```ahk
a := ComObjArray(0xB, 3)
```

## См. также

[`ComObject`](ComObject.md), [`ComValue`](ComValue.md)
