# `MethodError`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · class · Le cœur du langage : texte, mathématiques et motifs

## Description

A method called on a value that has no such method.

## Syntaxe

```ahk
e := MethodError(Message, What, Extra)
```

## Paramètres

?.

## Valeur de retour

An error object (Message/What/Extra/File/Line/Stack).

## Exemple

```ahk
throw MethodError("x")
```

## Voir aussi

[`Error`](Error.md), [`Throw`](Throw.md), [`OnError`](OnError.md)
