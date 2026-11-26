# `ObjGetBase`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · function · 言語のオブジェクト: Array、Map、Func

## 説明

The base of an object: the prototype of its class.

## 構文

```ahk
ObjGetBase …
```

## パラメータ

the object

## 戻り値

the object of the base

## エラー

a value that is not an object is refused with "Expected an object but got ...".

## 例

```ahk
FileAppend ObjGetBase({a: 1}).__Class, "*"
```
