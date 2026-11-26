# `IniDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Operazioni su file e directory

## Descrizione

Takes a key of an ini file away. A key that is not given takes the whole section with it.

## Sintassi

```ahk
IniDelete …
```

## Parametri

FileName, Section, [Key]

## Valore di ritorno

nothing

## Errori

Error when the file cannot be written.

## Esempio

```ahk
IniDelete "app.ini", "Main", "Name"
```
