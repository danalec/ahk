# `ObjGetBase`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · function · The objects of the language: Array, Map and Func

## Description

The base of an object: the prototype of its class.

## Syntax

```ahk
ObjGetBase …
```

## Parameters

the object

## Return value

the object of the base

## Errors

a value that is not an object is refused with "Expected an object but got ...".

## Example

```ahk
FileAppend ObjGetBase({a: 1}).__Class, "*"
```
