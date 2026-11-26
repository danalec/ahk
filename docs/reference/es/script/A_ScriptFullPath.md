# `A_ScriptFullPath`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · variable · El analizador y la ejecución del script

## Descripción

The path of the script. The script of the runner comes on the standard input and has no file, so the path is an asterisk.

## Sintaxis

```ahk
A_ScriptFullPath 
```

## Parámetros

nothing; it is a variable, not a call

## Valor de retorno

the text "*"

## Errores

the write is refused.

## Ejemplo

```ahk
FileAppend A_ScriptFullPath, "*"
```
