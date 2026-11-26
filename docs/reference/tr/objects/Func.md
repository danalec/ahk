# `Func`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · Dilin nesneleri: Array, Map ve Func

## Açıklama

The class of a function: a function of the script, a method, a lambda or a built-in. The class is not made: `Func(...)` answers "Invalid base."; the class of a function value is `Func`.

## Sözdizimi

```ahk
Func …
```

## Parametreler

nothing that makes it

## Dönüş değeri

the class of a function value, as a text

## Hatalar

the call of the class is refused with "Invalid base.".

## Örnek

```ahk
FileAppend Abs.__Class, "*"
```
