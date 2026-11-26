# `ObjGetBase`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · function · Les objets du langage : Array, Map et Func

## Description

The base of an object: the prototype of its class.

## Syntaxe

```ahk
ObjGetBase …
```

## Paramètres

the object

## Valeur de retour

the object of the base

## Erreurs

a value that is not an object is refused with "Expected an object but got ...".

## Exemple

```ahk
FileAppend ObjGetBase({a: 1}).__Class, "*"
```
