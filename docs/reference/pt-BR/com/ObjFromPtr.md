# `ObjFromPtr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · function · Automação COM (ComObject)

## Descrição

Wraps an object pointer as an object (no addref).

## Sintaxe

```ahk
Obj := ObjFromPtr(Ptr)
```

## Parâmetros

- **Ptr** — The object pointer.

## Valor de retorno

The object.

## Exemplo

```ahk
o := ObjFromPtr(p)
```

## Relacionados

[`ObjPtr`](ObjPtr.md), [`ObjFromPtrAddRef`](ObjFromPtrAddRef.md)
