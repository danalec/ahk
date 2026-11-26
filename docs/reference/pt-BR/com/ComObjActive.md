# `ComObjActive`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · Automação COM (ComObject)

## Descrição

Gets a running COM object by CLSID (the ROT).

## Sintaxe

```ahk
Obj := ComObjActive(CLSID)
```

## Parâmetros

- **CLSID** — The class id.

## Valor de retorno

The running object.

## Exemplo

```ahk
o := ComObjActive("{...}")
```

## Relacionados

[`ComObjGet`](ComObjGet.md), [`ComObject`](ComObject.md)
