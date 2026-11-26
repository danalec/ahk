# `GetMethod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · method · `Object` · Ядро языка: текст, математика и шаблоны

## Описание

The method of an object, as a value that can be called.

## Синтаксис

```ahk
GetMethod 
```

## Параметры

the object, the name of the method, and the number of parameters it would take

## Возвращаемое значение

the method, as a value of the class `Func`

## Ошибки

a method that is not there is refused with the type of the value and the name of the member.

## Пример

```ahk
o := {a: 1}
FileAppend GetMethod(o, "HasOwnProp").__Class, "*"
```
