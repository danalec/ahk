# `ComObjCreate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · Automação COM (ComObject)

## Descrição

Creates a COM object (v1).

## Sintaxe

```ahk
Obj := ComObjCreate(CLSID)
```

## Parâmetros

1..2.

## Valor de retorno

The object.

## Exemplo

```ahk
o := ComObjCreate("Shell.Application")
```

## Relacionados

[`ComObject`](ComObject.md)
