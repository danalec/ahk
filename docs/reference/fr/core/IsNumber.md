# `IsNumber`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · function · Le cœur du langage : texte, mathématiques et motifs

## Description

Whether a value is a number: an integer, a float, or a text that holds one. A hexadecimal text is a number too.

## Syntaxe

```ahk
B := IsNumber(Value)
```

## Paramètres

- **Value** — Any value.

## Valeur de retour

1 when it is a number, 0 when it is not

## Erreurs

a second parameter is refused with "the code page of the text is not carried yet.".

## Exemple

```ahk
IsNumber("42")
```

## Voir aussi

[`IsInteger`](IsInteger.md), [`IsFloat`](IsFloat.md)
