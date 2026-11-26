# `ComCall`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · function · COM-автоматизация (ComObject)

## Описание

Calls a COM method by its vtable index.

## Синтаксис

```ahk
R := ComCall(Index, Obj, Args..., "ReturnType")
```

## Параметры

- **Index** — The vtable slot.
- **Obj** — The COM object.
- **Args** — Type/value pairs.

## Возвращаемое значение

The return value.

## Замечания

On Linux the D-Bus model has no vtable index; refuses with the reason.

## Пример

```ahk
ComCall(0, obj, "Ptr")
```

## См. также

[`ComObject`](ComObject.md), [`ComObjQuery`](ComObjQuery.md)
