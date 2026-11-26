# `IniRead`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Operaciones de archivos y directorios

## Descripción

Reads the value of a key of an ini file. With no section it answers the names of the sections; with no key it answers the section itself. The ini files are the ones of Windows: `[section]`, `key=value`, and `;` for a comment.

## Sintaxis

```ahk
V := IniRead(Filename, Section, Key, Default)
```

## Parámetros

- **Filename** — The INI.
- **Section** — The section.
- **Key** — The key; omitted reads the whole section.
- **Default** — Returned when absent.

## Valor de retorno

the value of the key; the default of the script when the key is not there and a default is given.

## Errores

Error `The requested key, section or file was not found.` when the key is not there and no default is given.

## Ejemplo

```ahk
v := IniRead("app.ini", "Main", "Name")
```

## Relacionados

[`IniWrite`](IniWrite.md), [`IniDelete`](IniDelete.md)
