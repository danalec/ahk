# `IniRead`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · Datei- und Verzeichnisoperationen

## Beschreibung

Reads the value of a key of an ini file. With no section it answers the names of the sections; with no key it answers the section itself. The ini files are the ones of Windows: `[section]`, `key=value`, and `;` for a comment.

## Syntax

```ahk
V := IniRead(Filename, Section, Key, Default)
```

## Parameter

- **Filename** — The INI.
- **Section** — The section.
- **Key** — The key; omitted reads the whole section.
- **Default** — Returned when absent.

## Rückgabewert

the value of the key; the default of the script when the key is not there and a default is given.

## Fehler

Error `The requested key, section or file was not found.` when the key is not there and no default is given.

## Beispiel

```ahk
v := IniRead("app.ini", "Main", "Name")
```

## Verwandt

[`IniWrite`](IniWrite.md), [`IniDelete`](IniDelete.md)
