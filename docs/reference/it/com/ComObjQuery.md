# `ComObjQuery`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · function · Automazione COM (ComObject)

## Descrizione

Queries another interface of a COM object.

## Sintassi

```ahk
Obj := ComObjQuery(Obj, IID)
```

## Parametri

- **Obj** — The object.
- **IID** — The interface.

## Valore di ritorno

The interface object.

## Esempio

```ahk
q := ComObjQuery(o, "{...}")
```

## Correlati

[`ComObject`](ComObject.md), [`ComCall`](ComCall.md)
