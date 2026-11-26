# `FileOpen`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · Les opérations sur les fichiers et les répertoires

## Description

Opens a file and answers the object of it, of the class `File`. The modes are `r`, `w`, `a` and `rw`; the name `*` is the console of the machine.

## Syntaxe

```ahk
File := FileOpen(Filename, Flags, Encoding)
```

## Paramètres

- **Filename** — The path; * is the console.
- **Flags** — r/w/a/rw plus share mode.
- **Encoding** — UTF-8/UTF-16/CPnnn/RAW.

## Valeur de retour

a `File` object.

## Erreurs

ValueError when the mode is not one of the four; OSError when the file cannot be opened.

## Remarques

The File object keeps the handle; Close releases it.

## Exemple

```ahk
f := FileOpen("out.txt", "w")
```

## Voir aussi

[`FileRead`](FileRead.md), [`FileAppend`](FileAppend.md)
