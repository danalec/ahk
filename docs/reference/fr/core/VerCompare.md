# `VerCompare`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · Le cœur du langage : texte, mathématiques et motifs

## Description

Two version strings, compared. The numbers of the two versions are compared one by one, and the component that is missing is a zero. A version with a pre-release suffix is older than the same version without it.

## Syntaxe

```ahk
VerCompare …
```

## Paramètres

Version1, Version2

## Valeur de retour

the order of the two versions as -1, 0 or 1; when the second version carries an operator -- <, <=, >, >= or = -- the answer is whether the relation holds

## Erreurs

none.

## Exemple

```ahk
VerCompare("1.2.0", "1.10.0")
```
