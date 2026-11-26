# `GetMethod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · method · `Object` · Rdzeń języka: tekst, matematyka i wzorce

## Opis

The method of an object, as a value that can be called.

## Składnia

```ahk
GetMethod 
```

## Parametry

the object, the name of the method, and the number of parameters it would take

## Wartość zwracana

the method, as a value of the class `Func`

## Błędy

a method that is not there is refused with the type of the value and the name of the member.

## Przykład

```ahk
o := {a: 1}
FileAppend GetMethod(o, "HasOwnProp").__Class, "*"
```
