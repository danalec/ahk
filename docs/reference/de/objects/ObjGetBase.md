# `ObjGetBase`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · function · Die Objekte der Sprache: Array, Map und Func

## Beschreibung

The base of an object: the prototype of its class.

## Syntax

```ahk
ObjGetBase …
```

## Parameter

the object

## Rückgabewert

the object of the base

## Fehler

a value that is not an object is refused with "Expected an object but got ...".

## Beispiel

```ahk
FileAppend ObjGetBase({a: 1}).__Class, "*"
```
