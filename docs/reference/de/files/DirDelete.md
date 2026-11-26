# `DirDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · command · Datei- und Verzeichnisoperationen

## Beschreibung

Takes a directory away. A directory that is not empty stays without it unless the recursion is asked for.

## Syntax

```ahk
DirDelete Path, Recurse
```

## Parameter

- **Path** — The directory.
- **Recurse** — 1 to delete contents.

## Rückgabewert

nothing; the name of a command answers nothing.

## Fehler

Error `Failed` when the directory is not there, or when it is not empty and the recursion is not asked for.

## Beispiel

```ahk
d := DirDelete("out", 1)
```

## Verwandt

[`DirCreate`](DirCreate.md), [`FileDelete`](FileDelete.md)
