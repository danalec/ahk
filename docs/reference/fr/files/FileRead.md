# `FileRead`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Les opérations sur les fichiers et les répertoires

## Description

Reads a whole file and answers its text. The mark of the encoding decides the encoding of the file, and the option of the script decides when the file has no mark. The value of the command is taken with the parenthesis, the way `FileRead("out.txt")` is written: the command style inside an expression fails with "Expected a String but got a Func.", and the reference of v2.0.31 fails the same way.

## Syntaxe

```ahk
Text := FileRead(Filename, Options)
```

## Paramètres

- **Filename** — The path.
- **Options** — `n (LF), `r`n, RAW, encoding (UTF-8/UTF-16/CPnnn), m<n> limit.

## Valeur de retour

the text of the file; a Buffer when the option `RAW` is there.

## Erreurs

OSError when the file cannot be read; ValueError when an option is not one of v2. The options are the encoding of the file -- `UTF-8`, `UTF-16` and the code page of the machine --, `RAW` and `m<n>`, the number of bytes to read.

## Remarques

A missing file throws an OSError. The encoding is detected from a BOM, else the option, else the default.

## Exemple

```ahk
text := FileRead("out.txt")
```

## Voir aussi

[`FileAppend`](FileAppend.md), [`FileOpen`](FileOpen.md), [`LoopRead`](LoopRead.md)
