# `IniDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Les opérations sur les fichiers et les répertoires

## Description

Takes a key of an ini file away. A key that is not given takes the whole section with it.

## Syntaxe

```ahk
IniDelete …
```

## Paramètres

FileName, Section, [Key]

## Valeur de retour

nothing

## Erreurs

Error when the file cannot be written.

## Exemple

```ahk
IniDelete "app.ini", "Main", "Name"
```
