# `ComCall`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · function · Automação COM (ComObject)

## Descrição

Calls a COM method by its vtable index.

## Sintaxe

```ahk
R := ComCall(Index, Obj, Args..., "ReturnType")
```

## Parâmetros

- **Index** — The vtable slot.
- **Obj** — The COM object.
- **Args** — Type/value pairs.

## Valor de retorno

The return value.

## Observações

On Linux the D-Bus model has no vtable index; refuses with the reason.

## Exemplo

```ahk
ComCall(0, obj, "Ptr")
```

## Relacionados

[`ComObject`](ComObject.md), [`ComObjQuery`](ComObjQuery.md)
