# `FileSetAttrib`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Les opérations sur les fichiers et les répertoires

## Description

Changes the attributes of a file. The attributes come first and the name of the file second, the way the binary of v2.0.31 takes them. The name of the file may be a pattern, and the attributes are changed in every file that the pattern names.

## Syntaxe

```ahk
FileSetAttrib …
```

## Paramètres

Attributes, FileName, [Recurse]

## Valeur de retour

nothing

## Erreurs

ValueError when the attributes are not one of v2; OSError when the attributes of the file cannot be changed. The third parameter is the recursion: the "R" walks into the subfolders, and any other letter is refused.

## Exemple

```ahk
FileSetAttrib "+H", "*.txt"
```
