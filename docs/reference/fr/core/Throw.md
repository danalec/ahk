# `Throw`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*KeySharp* · function · Le cœur du langage : texte, mathématiques et motifs

## Description

A statement that raises a value. The value is usually an error object, and it may be any value.

## Syntaxe

```ahk
Throw 
```

## Paramètres

Value

## Valeur de retour

nothing; the statement leaves the flow of the script

## Erreurs

none; the value is caught by the catch that names its class.

## Exemple

```ahk
throw ValueError("boom")
```
