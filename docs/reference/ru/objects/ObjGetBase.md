# `ObjGetBase`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · function · Объекты языка: Array, Map и Func

## Описание

The base of an object: the prototype of its class.

## Синтаксис

```ahk
ObjGetBase …
```

## Параметры

the object

## Возвращаемое значение

the object of the base

## Ошибки

a value that is not an object is refused with "Expected an object but got ...".

## Пример

```ahk
FileAppend ObjGetBase({a: 1}).__Class, "*"
```
