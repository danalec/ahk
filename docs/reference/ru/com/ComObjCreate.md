# `ComObjCreate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · COM-автоматизация (ComObject)

## Описание

Creates a COM object (v1).

## Синтаксис

```ahk
Obj := ComObjCreate(CLSID)
```

## Параметры

1..2.

## Возвращаемое значение

The object.

## Пример

```ahk
o := ComObjCreate("Shell.Application")
```

## См. также

[`ComObject`](ComObject.md)
