# `Enumerator`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · class · Dilin nesneleri: Array, Map ve Func

## Açıklama

The class of the walker of a loop: an enumerator hands out the pieces of a sequence, one turn at a time. The class is not made; the `__Enum` of a value makes one, and the call of it is the turn of the enumeration.

## Sözdizimi

```ahk
Enumerator 
```

## Parametreler

nothing that makes it

## Dönüş değeri

the class of an enumerator, as a text

## Hatalar

the call of the class is refused with "Invalid base.".

## Örnek

```ahk
for item in [1, 2]
    FileAppend Type(item), "*"
```
