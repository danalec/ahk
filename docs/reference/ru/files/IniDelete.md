# `IniDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Операции с файлами и каталогами

## Описание

Takes a key of an ini file away. A key that is not given takes the whole section with it.

## Синтаксис

```ahk
IniDelete …
```

## Параметры

FileName, Section, [Key]

## Возвращаемое значение

nothing

## Ошибки

Error when the file cannot be written.

## Пример

```ahk
IniDelete "app.ini", "Main", "Name"
```
