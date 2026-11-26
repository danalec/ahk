# `A_IsAdmin`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · variable · Разбор и выполнение скрипта

## Описание

Whether the script runs with the rights of an administrator: the effective user id, and the root is the one that is zero.

## Синтаксис

```ahk
A_IsAdmin 
```

## Параметры

nothing; it is a variable, not a call

## Возвращаемое значение

1 when the script is elevated, 0 when it is not

## Ошибки

the read fails when the machine cannot say; the write is refused.

## Пример

```ahk
FileAppend A_IsAdmin, "*"
```
