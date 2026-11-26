# `A_ScriptFullPath`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · variable · Parser i wykonanie skryptu

## Opis

The path of the script. The script of the runner comes on the standard input and has no file, so the path is an asterisk.

## Składnia

```ahk
A_ScriptFullPath 
```

## Parametry

nothing; it is a variable, not a call

## Wartość zwracana

the text "*"

## Błędy

the write is refused.

## Przykład

```ahk
FileAppend A_ScriptFullPath, "*"
```
