# `IniWrite`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Operaciones de archivos y directorios

## Descripción

Writes a key of an ini file. The value comes first and the name of the file second, the way the binary of v2.0.31 takes them; a file that is not there is made.

## Sintaxis

```ahk
IniWrite Value, Filename, Section, Key
```

## Parámetros

- **Value** — The text.
- **Filename** — The INI.
- **Section** — The section.
- **Key** — The key.

## Valor de retorno

nothing

## Errores

Error when the file cannot be written.

## Ejemplo

```ahk
IniWrite "hello", "app.ini", "Main", "Name"
```

## Relacionados

[`IniRead`](IniRead.md), [`IniDelete`](IniDelete.md)
