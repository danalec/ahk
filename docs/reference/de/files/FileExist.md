# `FileExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · function · Datei- und Verzeichnisoperationen

## Beschreibung

Answers the attributes of the first entry that the name or the pattern matches. The value of the command is taken with the parenthesis, the way `FileExist("a.txt")` is written.

## Syntax

```ahk
Attrib := FileExist(Path)
```

## Parameter

- **Path** — A path or a wildcard pattern.

## Rückgabewert

the attributes as letters: `A` for a file, `D` for a folder, `RA` for a file that cannot be written, and the empty text when there is no such file.

## Fehler

nothing fails: a name that matches nothing answers the empty text.

## Bemerkungen

A wildcard answers the first match in the folder's enumeration order.

## Beispiel

```ahk
x := FileExist("out.txt")
```

## Verwandt

[`DirExist`](DirExist.md), [`FileGetAttrib`](FileGetAttrib.md)
