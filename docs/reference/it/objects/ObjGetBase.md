# `ObjGetBase`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · function · Gli oggetti del linguaggio: Array, Map e Func

## Descrizione

The base of an object: the prototype of its class.

## Sintassi

```ahk
ObjGetBase …
```

## Parametri

the object

## Valore di ritorno

the object of the base

## Errori

a value that is not an object is refused with "Expected an object but got ...".

## Esempio

```ahk
FileAppend ObjGetBase({a: 1}).__Class, "*"
```
