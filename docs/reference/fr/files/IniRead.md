# `IniRead`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Les opérations sur les fichiers et les répertoires

## Description

Reads the value of a key of an ini file. With no section it answers the names of the sections; with no key it answers the section itself. The ini files are the ones of Windows: `[section]`, `key=value`, and `;` for a comment.

## Syntaxe

```ahk
V := IniRead(Filename, Section, Key, Default)
```

## Paramètres

- **Filename** — The INI.
- **Section** — The section.
- **Key** — The key; omitted reads the whole section.
- **Default** — Returned when absent.

## Valeur de retour

the value of the key; the default of the script when the key is not there and a default is given.

## Erreurs

Error `The requested key, section or file was not found.` when the key is not there and no default is given.

## Exemple

```ahk
v := IniRead("app.ini", "Main", "Name")
```

## Voir aussi

[`IniWrite`](IniWrite.md), [`IniDelete`](IniDelete.md)
