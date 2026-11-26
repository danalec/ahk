# `HasMethod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · method · `Object` · Ядро языка: текст, математика и шаблоны

## Описание

Whether an object has a method.

## Синтаксис

```ahk
HasMethod 
```

## Параметры

the object, the name of the method, and the number of parameters it would take

## Возвращаемое значение

1 when the object has such a method, 0 when it has not

## Ошибки

nothing; a value that is not an object answers zero, and the question with no name asks whether the value is a class.

## Пример

```ahk
o := {a: 1}
FileAppend HasMethod(o, "a"), "*"
```
