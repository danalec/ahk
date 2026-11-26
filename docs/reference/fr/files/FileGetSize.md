# `FileGetSize`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Les opérations sur les fichiers et les répertoires

## Description

Answers how big a file is.

## Syntaxe

```ahk
FileGetSize …
```

## Paramètres

FileName, [Unit]

## Valeur de retour

the size, rounded down; a folder answers zero.

## Erreurs

ValueError when the unit is not `B`, `K` or `M`; OSError when the file cannot be read.

## Exemple

```ahk
n := FileGetSize("out.txt", "K")
```
