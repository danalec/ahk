# `BlockInput`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Горячие клавиши, горячие строки и ввод

## Описание

Turns the input of the whole machine off, and that is all it can do. Turning input off for the whole machine is a privilege this compositor does not give a client, and this tree will not claim it did.

## Синтаксис

```ahk
BlockInput …
```

## Параметры

Mode

## Возвращаемое значение

nothing for 'Off'

## Ошибки

any other mode fails with 'blocking the input of the whole machine is a privilege this compositor does not give a client'.

## Пример

```ahk
BlockInput "Off"
```
