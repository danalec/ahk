# `IniDelete`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Operaciones de archivos y directorios

## Descripción

Takes a key of an ini file away. A key that is not given takes the whole section with it.

## Sintaxis

```ahk
IniDelete …
```

## Parámetros

FileName, Section, [Key]

## Valor de retorno

nothing

## Errores

Error when the file cannot be written.

## Ejemplo

```ahk
IniDelete "app.ini", "Main", "Name"
```
