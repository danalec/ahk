# `FileGetSize`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Операции с файлами и каталогами

## Описание

Answers how big a file is.

## Синтаксис

```ahk
FileGetSize …
```

## Параметры

FileName, [Unit]

## Возвращаемое значение

the size, rounded down; a folder answers zero.

## Ошибки

ValueError when the unit is not `B`, `K` or `M`; OSError when the file cannot be read.

## Пример

```ahk
n := FileGetSize("out.txt", "K")
```
