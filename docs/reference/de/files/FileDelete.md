# `FileDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · Datei- und Verzeichnisoperationen

## Beschreibung

Takes a file away, or every file that a pattern names. A pattern that matches nothing is not a failure.

## Syntax

```ahk
FileDelete Pattern
```

## Parameter

- **Pattern** — A path or wildcard.

## Rückgabewert

nothing

## Fehler

OSError when the file cannot be taken away.

## Beispiel

```ahk
FileDelete "out.txt"
```

## Verwandt

[`DirDelete`](DirDelete.md), [`FileRecycle`](FileRecycle.md)
