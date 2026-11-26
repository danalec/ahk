# `ObjRelease`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · function · Automação COM (ComObject)

## Descrição

Releases a reference to an object pointer.

## Sintaxe

```ahk
ObjRelease Ptr
```

## Parâmetros

- **Ptr** — The pointer.

## Valor de retorno

The new reference count.

## Exemplo

```ahk
ObjRelease p
```

## Relacionados

[`ObjPtrAddRef`](ObjPtrAddRef.md), [`ObjFromPtr`](ObjFromPtr.md)
