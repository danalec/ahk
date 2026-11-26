# `HasProp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · method · `Object` · Le cœur du langage : texte, mathématiques et motifs

## Description

Whether an object has a property.

## Syntaxe

```ahk
B := HasProp(Obj, Name)
```

## Paramètres

- **Obj** — The object.
- **Name** — The property.

## Valeur de retour

1 when the object has such a property, 0 when it has not

## Erreurs

nothing; a value that is not an object has the property `__Class` and no other, so the answer for it is one only for that name.

## Exemple

```ahk
o := {a: 1}
FileAppend HasProp(o, "a"), "*"
```

## Voir aussi

[`HasMethod`](HasMethod.md), [`DefineProp`](DefineProp.md), [`OwnProps`](OwnProps.md)
