# `IsSet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · Разбор и выполнение скрипта

## Описание

Whether the variable of the caller has a value. The tree carries it besides `IsSetRef`, and it asks for the variable and not for its value.

## Синтаксис

```ahk
IsSet …
```

## Параметры

the variable, not its value

## Возвращаемое значение

1 when the variable has a value, 0 when it has none

## Ошибки

an argument that is not a variable is refused with "IsSet requires a variable.".

## Пример

```ahk
x := 1
FileAppend IsSet(x), "*"
```
