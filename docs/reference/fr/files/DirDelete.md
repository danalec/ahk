# `DirDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · command · Les opérations sur les fichiers et les répertoires

## Description

Takes a directory away. A directory that is not empty stays without it unless the recursion is asked for.

## Syntaxe

```ahk
DirDelete Path, Recurse
```

## Paramètres

- **Path** — The directory.
- **Recurse** — 1 to delete contents.

## Valeur de retour

nothing; the name of a command answers nothing.

## Erreurs

Error `Failed` when the directory is not there, or when it is not empty and the recursion is not asked for.

## Exemple

```ahk
d := DirDelete("out", 1)
```

## Voir aussi

[`DirCreate`](DirCreate.md), [`FileDelete`](FileDelete.md)
