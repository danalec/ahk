# `Enumerator`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · class · Gli oggetti del linguaggio: Array, Map e Func

## Descrizione

The class of the walker of a loop: an enumerator hands out the pieces of a sequence, one turn at a time. The class is not made; the `__Enum` of a value makes one, and the call of it is the turn of the enumeration.

## Sintassi

```ahk
Enumerator 
```

## Parametri

nothing that makes it

## Valore di ritorno

the class of an enumerator, as a text

## Errori

the call of the class is refused with "Invalid base.".

## Esempio

```ahk
for item in [1, 2]
    FileAppend Type(item), "*"
```
