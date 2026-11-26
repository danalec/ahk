# `DirCreate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · command · Les opérations sur les fichiers et les répertoires

## Description

Makes a directory, with the folders that are missing in the way.

## Syntaxe

```ahk
DirCreate Path
```

## Paramètres

- **Path** — The directory.

## Valeur de retour

nothing

## Erreurs

OSError when the directory cannot be made.

## Exemple

```ahk
DirCreate "out"
```

## Voir aussi

[`DirDelete`](DirDelete.md), [`DirCopy`](DirCopy.md)
