# `ComObjFromPtr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · function · Automação COM (ComObject)

## Descrição

Wraps a COM interface pointer as an object.

## Sintaxe

```ahk
Obj := ComObjFromPtr(Ptr)
```

## Parâmetros

- **Ptr** — The interface pointer.

## Valor de retorno

The object.

## Exemplo

```ahk
o := ComObjFromPtr(p)
```

## Relacionados

[`ObjPtr`](ObjPtr.md), [`ComObjQuery`](ComObjQuery.md)
