# `DirExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · command · Datei- und Verzeichnisoperationen

## Beschreibung

Answers the attributes of the first folder that the name or the pattern matches.

## Syntax

```ahk
A := DirExist(Path)
```

## Parameter

- **Path** — A path or wildcard.

## Rückgabewert

`D` when the folder is there, and the empty text when it is not; a file that matches the pattern is not a folder.

## Fehler

nothing fails.

## Beispiel

```ahk
x := DirExist("out")
```

## Verwandt

[`FileExist`](FileExist.md)
