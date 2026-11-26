# `ComObjType`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · Automação COM (ComObject)

## Descrição

Reads the type/name of a COM object.

## Sintaxe

```ahk
S := ComObjType(Obj, What)
```

## Parâmetros

- **Obj** — The object.
- **What** — Name/Class/IID/...

## Valor de retorno

The requested string.

## Exemplo

```ahk
n := ComObjType(o, "Name")
```

## Relacionados

[`ComObject`](ComObject.md), [`ComObjValue`](ComObjValue.md)
