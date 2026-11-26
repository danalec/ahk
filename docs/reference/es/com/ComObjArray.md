# `ComObjArray`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · Automatización COM (ComObject)

## Descripción

Creates a COM safe array.

## Sintaxis

```ahk
A := ComObjArray(Type, Counts...)
```

## Parámetros

2..255.

## Valor de retorno

A COM array.

## Ejemplo

```ahk
a := ComObjArray(0xB, 3)
```

## Relacionados

[`ComObject`](ComObject.md), [`ComValue`](ComValue.md)
