# `HasBase`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · method · `Object` · Le cœur du langage : texte, mathématiques et motifs

## Description

Whether the base of an object is another object.

## Syntaxe

```ahk
HasBase 
```

## Paramètres

the object, and the base

## Valeur de retour

1 when the base is that one, 0 when it is not

## Erreurs

a base that is not an object is refused before the question of the object is asked.

## Exemple

```ahk
FileAppend HasBase({a: 1}, {}), "*"
```
