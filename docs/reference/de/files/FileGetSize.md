# `FileGetSize`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · Datei- und Verzeichnisoperationen

## Beschreibung

Answers how big a file is.

## Syntax

```ahk
FileGetSize …
```

## Parameter

FileName, [Unit]

## Rückgabewert

the size, rounded down; a folder answers zero.

## Fehler

ValueError when the unit is not `B`, `K` or `M`; OSError when the file cannot be read.

## Beispiel

```ahk
n := FileGetSize("out.txt", "K")
```
