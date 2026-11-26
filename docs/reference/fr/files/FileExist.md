# `FileExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · Les opérations sur les fichiers et les répertoires

## Description

Answers the attributes of the first entry that the name or the pattern matches. The value of the command is taken with the parenthesis, the way `FileExist("a.txt")` is written.

## Syntaxe

```ahk
Attrib := FileExist(Path)
```

## Paramètres

- **Path** — A path or a wildcard pattern.

## Valeur de retour

the attributes as letters: `A` for a file, `D` for a folder, `RA` for a file that cannot be written, and the empty text when there is no such file.

## Erreurs

nothing fails: a name that matches nothing answers the empty text.

## Remarques

A wildcard answers the first match in the folder's enumeration order.

## Exemple

```ahk
x := FileExist("out.txt")
```

## Voir aussi

[`DirExist`](DirExist.md), [`FileGetAttrib`](FileGetAttrib.md)
