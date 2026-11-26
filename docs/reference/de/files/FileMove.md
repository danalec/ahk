# `FileMove`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · Datei- und Verzeichnisoperationen

## Beschreibung

Moves a file, or every file that a pattern names. The source and the destination are the ones of `FileCopy`.

## Syntax

```ahk
FileMove Source, Dest, Overwrite
```

## Parameter

- **Source** — Path/wildcard.
- **Dest** — Destination.
- **Overwrite** — 1 to overwrite.

## Rückgabewert

nothing

## Fehler

Error `Failed` when the source is not there, or when the destination is there and the overwrite is not asked for.

## Beispiel

```ahk
FileMove "a.txt", "b.txt"
```

## Verwandt

[`FileCopy`](FileCopy.md), [`DirMove`](DirMove.md)
