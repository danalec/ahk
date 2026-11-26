# `GetMethod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · method · `Object` · Le cœur du langage : texte, mathématiques et motifs

## Description

The method of an object, as a value that can be called.

## Syntaxe

```ahk
GetMethod 
```

## Paramètres

the object, the name of the method, and the number of parameters it would take

## Valeur de retour

the method, as a value of the class `Func`

## Erreurs

a method that is not there is refused with the type of the value and the name of the member.

## Exemple

```ahk
o := {a: 1}
FileAppend GetMethod(o, "HasOwnProp").__Class, "*"
```
