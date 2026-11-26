# `A_ScriptFullPath`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · variable · Parser and script execution

## Description

The path of the script. The script of the runner comes on the standard input and has no file, so the path is an asterisk.

## Syntax

```ahk
A_ScriptFullPath 
```

## Parameters

nothing; it is a variable, not a call

## Return value

the text "*"

## Errors

the write is refused.

## Example

```ahk
FileAppend A_ScriptFullPath, "*"
```
