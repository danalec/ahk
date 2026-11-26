# `Func`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · Объекты языка: Array, Map и Func

## Описание

The class of a function: a function of the script, a method, a lambda or a built-in. The class is not made: `Func(...)` answers "Invalid base."; the class of a function value is `Func`.

## Синтаксис

```ahk
Func …
```

## Параметры

nothing that makes it

## Возвращаемое значение

the class of a function value, as a text

## Ошибки

the call of the class is refused with "Invalid base.".

## Пример

```ahk
FileAppend Abs.__Class, "*"
```
