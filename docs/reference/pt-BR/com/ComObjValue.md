# `ComObjValue`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · Automação COM (ComObject)

## Descrição

Reads the underlying pointer/value of a COM wrapper.

## Sintaxe

```ahk
P := ComObjValue(Obj)
```

## Parâmetros

- **Obj** — The wrapper.

## Valor de retorno

The pointer or value.

## Exemplo

```ahk
p := ComObjValue(o)
```

## Relacionados

[`ComObjFlags`](ComObjFlags.md), [`ObjPtr`](ObjPtr.md)
