# `Throw`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*KeySharp* · function · Ядро языка: текст, математика и шаблоны

## Описание

A statement that raises a value. The value is usually an error object, and it may be any value.

## Синтаксис

```ahk
Throw 
```

## Параметры

Value

## Возвращаемое значение

nothing; the statement leaves the flow of the script

## Ошибки

none; the value is caught by the catch that names its class.

## Пример

```ahk
throw ValueError("boom")
```
