# `DirCopy`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · command · Datei- und Verzeichnisoperationen

## Beschreibung

Copies a directory and everything in it. What is copied is the content of the directory into the destination.

## Syntax

```ahk
DirCopy …
```

## Parameter

Source, Dest, [Overwrite]

## Rückgabewert

nothing

## Fehler

Error `Failed` when the source is not a directory, or when the destination is there and the overwrite is not asked for; a name that holds a wildcard is refused, and the reference fails with `Failed` on a wildcard too.

## Beispiel

```ahk
DirCopy "src", "dst"
```
