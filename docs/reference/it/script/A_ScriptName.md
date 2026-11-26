# `A_ScriptName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · variable · L'analizzatore e l'esecuzione dello script

## Descrizione

The name of the script. The script of the runner comes on the standard input and has no file, so the name is an asterisk. A script may write the name, and the value written is the one it keeps.

## Sintassi

```ahk
A_ScriptName 
```

## Parametri

nothing; it is a variable, not a call

## Valore di ritorno

the text "*", or the text that was written

## Errori

nothing to read; the write takes any value.

## Esempio

```ahk
FileAppend A_ScriptName, "*"
```
