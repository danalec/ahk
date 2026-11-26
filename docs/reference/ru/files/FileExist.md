# `FileExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · Операции с файлами и каталогами

## Описание

Answers the attributes of the first entry that the name or the pattern matches. The value of the command is taken with the parenthesis, the way `FileExist("a.txt")` is written.

## Синтаксис

```ahk
Attrib := FileExist(Path)
```

## Параметры

- **Path** — A path or a wildcard pattern.

## Возвращаемое значение

the attributes as letters: `A` for a file, `D` for a folder, `RA` for a file that cannot be written, and the empty text when there is no such file.

## Ошибки

nothing fails: a name that matches nothing answers the empty text.

## Замечания

A wildcard answers the first match in the folder's enumeration order.

## Пример

```ahk
x := FileExist("out.txt")
```

## См. также

[`DirExist`](DirExist.md), [`FileGetAttrib`](FileGetAttrib.md)
