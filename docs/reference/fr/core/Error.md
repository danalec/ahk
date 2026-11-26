# `Error`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · class · Le cœur du langage : texte, mathématiques et motifs

## Description

The base class of the errors of the language.

## Syntaxe

```ahk
e := Error(Message, What, Extra)
```

## Paramètres

?.

## Valeur de retour

An error object (Message/What/Extra/File/Line/Stack).

## Exemple

```ahk
throw Error("x")
```

## Voir aussi

[`Error`](Error.md), [`Throw`](Throw.md), [`OnError`](OnError.md)
