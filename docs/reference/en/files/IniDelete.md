# `IniDelete`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · File and directory operations

## Description

Takes a key of an ini file away. A key that is not given takes the whole section with it.

## Syntax

```ahk
IniDelete …
```

## Parameters

FileName, Section, [Key]

## Return value

nothing

## Errors

Error when the file cannot be written.

## Example

```ahk
IniDelete "app.ini", "Main", "Name"
```
